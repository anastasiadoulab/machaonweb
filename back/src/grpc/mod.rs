use std::env;
use prost::bytes::BytesMut;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use anyhow::Result;
use tokio::io::AsyncWriteExt;
use tracing::debug;
use crate::jobreceiver::uncached_data::SyncData;
use crate::jobreceiver::{job_receiver_client::JobReceiverClient, JobRequest, StatusRequest, JobStatus, ResultRequest, self, JobDetails, ServerStatus, UncachedData};
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Identity};
use tonic::Request;

/*

This module handles the gRPC-based communication over mutual TLS between the root node and the rest in the MachaonWeb
distributed computational network.

*/

#[derive(Debug)]
pub struct GrpcClient{
    url: String,
    tls_config: ClientTlsConfig
}

impl GrpcClient{

    // Create a new instance
    pub fn new(endpoint: String, domain: &str) -> Result<Self> { 
        let data_dir = std::path::PathBuf::from_iter([env::var("MTLS_CERTS_PATH")?]);
        let server_root_ca_cert = std::fs::read_to_string(data_dir.join("machaonlocalca.cert"))?;
        let server_root_ca_cert = Certificate::from_pem(server_root_ca_cert);
        let client_cert = std::fs::read_to_string(data_dir.join("node0.cert"))?;
        let client_key = std::fs::read_to_string(data_dir.join("node0.key"))?;
        let client_identity = Identity::from_pem(client_cert, client_key);
    
        let tls = ClientTlsConfig::new()
            .domain_name(domain)
            .ca_certificate(server_root_ca_cert)
            .identity(client_identity);

        Ok(Self { url: endpoint, tls_config: tls.into() })
    }

    // Establish a secure connection with mutual TLS protocol
    pub async fn establish_secure_connection(&self) -> Result<JobReceiverClient<Channel>>  { 
        let channel = Channel::from_static(Box::leak(self.url.to_string().into_boxed_str()))
        .tls_config(self.tls_config.to_owned())?
        .connect()
        .await?;

        Ok(JobReceiverClient::new(channel))
    }

    // Establish an unencrypted connection
    pub async fn establish_insecure_connection(&self) -> Result<JobReceiverClient<Channel>>  { 
        let client = JobReceiverClient::connect(self.url.clone()).await?;

        Ok(client)
    }

    // Alias function for debugging purposes
    pub async fn establish_connection(&self) -> Result<JobReceiverClient<Channel>>  {  
        //Ok(Self::establish_insecure_connection(&self).await?)
        Ok(Self::establish_secure_connection(&self).await?)
    }

    // Query the status of a computing node in MachaonWeb 
    pub async fn get_server_status(&self) -> Result<i32> {
        let mut client = Self::establish_connection(&self).await?;

        let request = tonic::Request::new(StatusRequest {});
        let response = client.get_status(request).await?;

        Ok(response.into_inner().status_code)
    }

    // Assign a job to a node
    pub async fn start_job(&self, job_request : &JobRequest) -> Result<JobStatus> {
        let mut client = Self::establish_connection(&self).await?;

        let request = tonic::Request::new(job_request.clone());
        let response = client.start_job(request).await?;

        Ok(response.into_inner())
    }

    // Download the result of a finished job from a node
    pub async fn download_result(&self, result_request : &ResultRequest, file_path: &str) -> Result<JobDetails> {
        let mut client = Self::establish_connection(&self).await?;
        let request = tonic::Request::new( result_request.clone());
        // Calling proto method
        let mut response = client.download_result(request).await?.into_inner();
        // Create a file to store the downloaded data
        let mut result_archive = tokio::fs::OpenOptions::new()
                                        .append(true)
                                        .create(true)
                                        .open(&file_path)
                                        .await?;
        let mut job_info = JobDetails{request_id: -1, hash: String::from(""), secure_hash: String::from(""), status_code: -1};

        // Download data via streaming using chunks
        let mut streaming_finished = false;
        while let Some(stream_chunk) = response.message().await?{
            match stream_chunk.job_data {
                Some(jobreceiver::job_result::JobData::FileInfo(info)) => {job_info = info.clone();}
                Some(jobreceiver::job_result::JobData::ChunkData(file_chunk)) => {
                    result_archive.write_all(&file_chunk).await?; 
                }
                None =>  { streaming_finished = true; }
            };
            if streaming_finished == true{
                break;
            }
        }
        Ok(job_info)
    }

    // Transfer data to the node
    pub async fn synchronize_node(&self, data_path: &str, secure_hash: String) -> Result<ServerStatus> {
        let mut client = Self::establish_connection(&self).await?;
        let mut archive = File::open(data_path).await?;
        let eof : usize = 0;

        // Create a stream for the upload including the archive file and its hash
        let stream = async_stream::stream! {
            let mut uncached_data = UncachedData::default();
            uncached_data.sync_data = Some(SyncData::SecureHash(secure_hash));
            yield uncached_data;
            loop {
                let mut buf = BytesMut::with_capacity(1024);
                match &archive.read_buf(&mut buf).await {
                    Ok(n) if *n == eof => {
                        break;
                    },
                    Ok(_n) => {
                        let mut uncached_data = UncachedData::default();
                        uncached_data.sync_data = Some(SyncData::ChunkData(buf.to_vec()));
                        yield uncached_data;
                    },
                    Err(e) => {
                        debug!("{}", e);
                        break;
                    },
                }
            } 
        };
        
        // Send the stream to the node
        let request = Request::new(stream);
        let response = client.synchronize(request).await?.into_inner();

        Ok(response)
    }


}