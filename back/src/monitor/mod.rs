use std::{ thread, time::Duration, i8, fs, path::{Path, self}, env };
use dotenvy::dotenv;
use rand::Rng;
use uuid::Uuid;
use crate::{ grpc::GrpcClient, db::{dbhandler::DatabaseHandler, models::{ NewJob, Node, QueriedRequest, Uncached }}, jobreceiver::{JobRequest, JobStatus, ResultRequest, JobDetails, ServerStatus}, utils };
use anyhow::Result;
use tracing::{debug, instrument};
use glob::{glob_with, MatchOptions};
use chrono::Utc;

/*  
This module is dedicated to handling the events of MachaonWeb framework:
- Request monitoring: new request creation
- Job monitoring: the handling of the active jobs and their finalization
- Computing node synchronization: each node in the network receives newer data
  that were produced from previous jobs in other nodes 
*/

#[derive(Debug)]
pub struct Monitor<'a>{ 
    request_monitoring_interval: Duration,
    job_monitoring_interval: Duration,
    sync_interval: Duration,
    db_handler: DatabaseHandler,
    root_dir_path: &'a str,
    output_dir_path: &'a str,
    protocol: &'a str
}

impl<'a> Monitor<'a> {

    // Create a new struct instance
    pub fn new<'b>(working_directory_full_path: &'a str, output_full_path: &'a str, web_protocol: &'a str,) -> Result<Monitor<'a>> {
        // Database handler
        let db_handler = DatabaseHandler::new()?;
        
        // Creating directories for saving extracted features and downloaded structures
        if Path::new(&working_directory_full_path).exists() == false {
            fs::create_dir_all(working_directory_full_path)?;
            fs::create_dir_all([working_directory_full_path, &path::MAIN_SEPARATOR.to_string(), "PDBs_new"].join(""))?;
            fs::create_dir_all([working_directory_full_path, &path::MAIN_SEPARATOR.to_string(), "DATA_PDBs_new_whole"].join(""))?;
            fs::create_dir_all([working_directory_full_path, &path::MAIN_SEPARATOR.to_string(), "DATA_PDBs_new_domain"].join(""))?;
        }

        // Creating directory the output directory where all the results are stored
        if Path::new(&output_full_path).exists() == false {
            fs::create_dir_all(output_full_path)?;
        }

        dotenv().ok();  
        Ok(Self{ db_handler: db_handler,
                request_monitoring_interval: Duration::from_secs(env::var("REQUEST_MONITORING_INTERVAL")?.parse::<u64>()?),
                job_monitoring_interval: Duration::from_secs(env::var("JOB_MONITORING_INTERVAL")?.parse::<u64>()?),
                sync_interval: Duration::from_secs(env::var("NODE_SYNC_INTERVAL")?.parse::<u64>()?), 
                root_dir_path: working_directory_full_path,
                output_dir_path: output_full_path,
                protocol: web_protocol })
    } 

    // Loop for monitoring new requests
    pub async fn monitor_requests(&mut self) {
        loop{
            match self.fulfill_request().await{
                Ok(result) => result,
                Err(e) => {debug!("{}", e); false}
            };
            thread::sleep(self.request_monitoring_interval);
        }
    }

    // Loop for checking running jobs
    pub async fn monitor_jobs(&mut self) {
        loop{
            match self.check_job().await{
                Ok(result) => result,
                Err(e) => {debug!("{}", e); false}
            };
            thread::sleep(self.job_monitoring_interval);
        }
    }

    // Loop for synchronizing nodes
    pub async fn sync_nodes (&mut self) {
        loop{
            match self.sync_node().await{
                Ok(result) => result,
                Err(e) => {debug!("{}", e); false}
            };
            thread::sleep(self.sync_interval);
        }
    }

    // Handling a new request by assigning a new job in an available computational node
    #[instrument(level="debug")]
    pub async fn fulfill_request(&mut self) -> Result<bool> {
        let db_handler = &mut self.db_handler;
        let mut result = false;
        // Check if there any nodes available
        let nodes_count = match db_handler.get_available_node_count().await?{
            Some(count) => count,
            None => 0
        };
        if nodes_count > 0
        {
            // Check if there are any requests that have not been handled yet
            let entries = db_handler.get_early_pending_request().await?;
            if entries.len() > 0 {
                let initialized = QueriedRequest::construct()?;
                let request = match entries.get(0){
                    Some(entry) => entry,
                    None => &initialized
                }; 
                if request.id > 0
                { 
                    // Double-check if the request has not been already fullfilled 
                    let secure_hash: String = match db_handler.get_fullfilled_request(&request.hash_value, 
                                                                                      &request.meta, &request.go_term).await?{ 
                            Some(sha_hash) => sha_hash,
                            None => String::from("") 
                    };
                    if secure_hash.len() > 0 {
                        // The request has already been processed
                        let new_job = NewJob{ request_id: &request.id, node_id: &-1, status_code: &0, completion_date:
                                                       Some(Utc::now().naive_utc()), secure_hash: &secure_hash};
                        DatabaseHandler::insert_job(db_handler, &new_job).await?;
                    }
                    else {
                        // Parse the candidate selections
                        let list_name: Vec<String> = match &request.list_name {
                            Some(entry) => entry.split(" ")
                            .into_iter()
                            .map(|s| s.to_string())
                            .collect(),
                            None => Vec::<String>::new()
                        };
                        
                        // Create a job instance
                        let job_request = JobRequest { 
                            reference_id: request.reference.clone(),
                            request_id: request.id,
                            listname: match list_name.get(0){
                                Some(entry) => entry.to_string(),
                                None => String::from("")
                            },
                            structure_ids: request.custom_list.split(",")
                            .into_iter()
                            .map(|s| s.to_string())
                            .collect(),
                            meta_analysis: request.meta,
                            go_term: request.go_term.clone(),
                            hash: request.hash_value.clone(),
                            comparison_mode: i32::from(request.comparison_mode),
                            segment_start: i32::from(request.segment_start),
                            segment_end: i32::from(request.segment_end),
                            alignment_level: i32::from(request.alignment_level)
                        };

                        // Assign a job to a computational node
                        result = self.assign_job(&job_request).await?;
                    }
                }
            }
        }
        Ok(result)
    } 
 
    // Synchronize a node in the MachaonWeb network
    pub async fn sync_node(&mut self) -> Result<bool> {
        let db_handler = &mut self.db_handler;
        let mut result = false;
        // Pattern matching options for feature filenames
        let options = MatchOptions {
            case_sensitive: true,
            require_literal_separator: false,
            require_literal_leading_dot: false
        };
        // Look for an outdated computing node
        let outdated_node = match DatabaseHandler::get_outdated_node(db_handler).await?{
            Some(node) => node,
            None => Node::construct()
        };  
        // Handle outdated node
        if outdated_node.id > 0 { 
            // Find which structures were processed in previous jobs since the node's last update
            let custom_lists:Vec<Uncached> = DatabaseHandler::get_uncached_by_date(db_handler, 
                                                                    outdated_node.id, outdated_node.sync_date).await?;
            if custom_lists.len() > 0{
                // Setting the paths for the cached data and creating a temporary folder for the update
                let uuid = Uuid::new_v4(); 
                let temp_folder_name = uuid.to_string(); 
                let temp_path = [self.root_dir_path, &path::MAIN_SEPARATOR.to_string(), &temp_folder_name].join("");
                let pdb_path = [&temp_path, &path::MAIN_SEPARATOR.to_string(), "PDBs_new"].join("");
                let data_path = [&temp_path, &path::MAIN_SEPARATOR.to_string(), "DATA_PDBs_new_whole"].join("");
                let domain_data_path = [&temp_path, &path::MAIN_SEPARATOR.to_string(), "DATA_PDBs_new_domain"].join("");
                let root_pdb_path = [self.root_dir_path, &path::MAIN_SEPARATOR.to_string(), "PDBs_new"].join("");
                let root_data_path = [self.root_dir_path, &path::MAIN_SEPARATOR.to_string(), "DATA_PDBs_new_whole"].join("");
                let root_domain_data_path = [self.root_dir_path, &path::MAIN_SEPARATOR.to_string(), "DATA_PDBs_new_domain"].join("");
                let output_path = [&temp_path, ".zip"].join("");
                fs::create_dir_all(&temp_path)?;
                fs::create_dir_all(&pdb_path)?;
                fs::create_dir_all(&data_path)?;
                fs::create_dir_all(&domain_data_path)?;

                // For each list of a newer request than the current node's last update, 
                // copy the associated uncached data to a temporary folder
                for custom_list in custom_lists {
                    let ids = custom_list.uncached.split(",");
                    for structure_id in ids { 
                        // structure data files
                        if Path::new(&data_path).exists() {
                            fs::copy([&root_pdb_path, &path::MAIN_SEPARATOR.to_string(), structure_id, ".pdb"].join(""),
                                       [&pdb_path, &path::MAIN_SEPARATOR.to_string(), structure_id, ".pdb"].join(""))?; 
                        }

                        // whole structure extracted features
                        if Path::new(&root_data_path).exists() {
                            for entry in glob_with(&[&root_data_path, 
                                                                                &path::MAIN_SEPARATOR.to_string(), 
                                                                                structure_id, "*"].join(""), options)? {
                                if let Ok(path) = entry {
                                    let filename = match path.file_name() {
                                        Some(os_string) => match os_string.to_str() {
                                            Some(base_name) => base_name,
                                            None => ""
                                        },
                                        None => ""
                                    };

                                    match path.as_os_str().to_str() {
                                        Some(path_string) => {
                                            fs::copy(path_string, 
                                            &[&data_path, &path::MAIN_SEPARATOR.to_string(), filename].join(""))?;
                                        },
                                        None => {}
                                    }  
                                }
                            }
                        }

                        // domain-level extracted features
                        if Path::new(&root_domain_data_path).exists() {
                            for entry in glob_with(&[&root_domain_data_path, 
                                                                               &path::MAIN_SEPARATOR.to_string(), 
                                                                               structure_id, "*"].join(""), options)? {
                                if let Ok(path) = entry {
                                    let filename = match path.file_name() {
                                        Some(os_string) => match os_string.to_str() {
                                            Some(base_name) => base_name,
                                            None => ""
                                        },
                                        None => ""
                                    };

                                    match path.as_os_str().to_str() {
                                        Some(path_string) => {
                                            fs::copy(path_string, 
                                            &[&data_path, &path::MAIN_SEPARATOR.to_string(), filename].join(""))?;
                                        },
                                        None => {}
                                    }  
                                }
                            }
                        }
                    }
                }
                // create a compressed archive that contains the new data
                if Path::new(&output_path).exists() {
                    fs::remove_file(&output_path)?;
                }
                let zip_ready = utils::compress_directory(&temp_path, &output_path)?;
                if zip_ready == true {
                    // Create a file hash for data integrity verification
                    let secure_hash = utils::compute_file_hash(&output_path)?;
                    // Upload the archive via mTLS gRPC
                    let client = GrpcClient::new([self.protocol, outdated_node.ip.as_str()].join(""), outdated_node.domain.as_str())?;
                    let server_status = match client.synchronize_node(&output_path, secure_hash.to_owned()).await{
                        Ok(status) => status,
                        Err(e) => {debug!("{}", e); ServerStatus { status_code: -1} }
                    }; 
                    // Update the database with the new update timestamp
                    if server_status.status_code == 0 {
                        DatabaseHandler::update_node_sync_date(db_handler, &outdated_node.id).await?;
                        result = true;
                    }
                    // Clean up
                    if Path::new(&temp_path).exists() {
                        fs::remove_dir_all(&temp_path)?;
                    }
                    if Path::new(&output_path).exists() {
                        fs::remove_file(&output_path)?;
                    }
                } 
            }
        }
        Ok(result)
    } 

    // Handle finished jobs
    #[instrument(level="debug")]
    pub async fn check_job(&mut self) -> Result<bool> {
        // Database handler
        let db_handler = &mut self.db_handler;
        // Look for a running job
        let active_job = db_handler.get_running_job().await?;
        let result  = match active_job {
            Some(running_job) => {
                // Set the path of the folder that contains the results, using the hash of the request
                let file_path = [self.root_dir_path, &path::MAIN_SEPARATOR.to_string(),
                                         &running_job.hash_value, ".zip"].join("");
                // Query the status of the node that has the running
                let client = GrpcClient::new([self.protocol, running_job.node_ip.as_str()].join(""), running_job.node_domain.as_str())?;
                let status = match client.get_server_status().await {
                    Ok(code) => code,
                    Err(e) => {debug!("{}", e); -1}
                };
                if status == 1
                { 
                    // Download the result if the job has finished
                    let job_check = ResultRequest{ hash: running_job.hash_value.clone(), request_id: running_job.request_id };
                    let job_details = match client.download_result(&job_check, &file_path).await{
                        Ok(result) => result,
                        Err(e) => {debug!("{}", e); JobDetails{request_id: -1, hash: String::from(""), 
                                                                    secure_hash: String::from(""), status_code: -1}}
                    }; 
                    let mut job_finished = false;
                    let mut file_hash= String::from("");
                    // If the download was successful
                    if job_details.status_code == 0{
                        if Path::new(&file_path).exists(){
                            // Compute the hash of the file
                            file_hash = utils::compute_file_hash(&file_path)?;
                            if file_hash == job_details.secure_hash {
                                job_finished = true;
                                // Create an output directory
                                let output_directory: &str = &[self.output_dir_path, &path::MAIN_SEPARATOR.to_string(), 
                                                                &running_job.hash_value].join("");
                                if Path::new(&output_directory).exists() == false {
                                    fs::create_dir_all(output_directory)?;
                                }
                                // Selectively extract the files from the downloaded archive:
                                // - html outputs for quick view
                                // - uncached data: structure data files & extracted features
                                utils::extract_result_files(&file_path, &output_directory, "zip")?; 
                                let compressed_filepath = &[output_directory, &path::MAIN_SEPARATOR.to_string(), &running_job.hash_value, 
                                                                ".zip"].join("");
                                utils::extract_result_files(&compressed_filepath, &output_directory, "html")?;
                                let output_directory = &[self.root_dir_path, &path::MAIN_SEPARATOR.to_string(), "PDBs_new"].join("");
                                utils::extract_result_files(&file_path, &output_directory, "pdb")?; 
                                let output_directory = &[self.root_dir_path, &path::MAIN_SEPARATOR.to_string(), "DATA_PDBs_new_"].join("");
                                if running_job.comparison_mode == 0 || running_job.comparison_mode == 1 {
                                    let mut data_directory = "whole";
                                    if running_job.comparison_mode == 1 {
                                        data_directory = "domain";
                                    }    
                                    fs::create_dir_all(&[&output_directory, data_directory].join(""))?;
                                    utils::extract_result_files(&file_path, &[&output_directory, data_directory].join(""), "proto")?; 
                                }
                                // Clean up
                                fs::remove_file(&file_path)?;
                            }
                        }
                    }
                    if job_details.status_code == -2 || job_details.status_code == -3 || job_finished == true {
                        // Handle failed jobs
                        DatabaseHandler::finalize_job(db_handler, &running_job.id, &file_hash, &(job_details.status_code as i8)).await?;
                        DatabaseHandler::update_node_working_state(db_handler, &running_job.node_id, false).await?;
                    } 
                }
                else 
                { 
                    // Store the timestamp of the job check
                    DatabaseHandler::update_job_check(db_handler, &running_job.id).await?;
                } 
                true
            },
            None =>  false
        };
        

        Ok(result)
    } 

    // Assign a job to a computing node
    #[instrument(level="debug")]
    pub async fn assign_job(&mut self, job_request : &JobRequest) -> Result<bool>{
        let db_handler = &mut self.db_handler;
        // Get idle nodes
        let nodes = match DatabaseHandler::get_available_nodes(db_handler).await {
            Ok(node_set) => node_set,
            Err(e) => {println!("{}", e); Vec::<Node>::new()}
        }; 
        // Maximum retries for assignment
        let mut retries = 3; 
        if nodes.len() > 0{
            loop { 
                // Choose randomly one node of the currently available ones
                let mut random_index: usize = 0;
                if nodes.len() > 1
                {
                    random_index = rand::thread_rng().gen_range(0..nodes.len());
                }
                let random_node = &nodes[random_index];
                // Query the status of the selected node
                let client = GrpcClient::new([self.protocol, random_node.ip.as_str()].join(""), random_node.domain.as_str())?;
                let status = match client.get_server_status().await{
                    Ok(code) => code,
                    Err(e) => {debug!("{}", e); -1}
                };
                retries -= 1;
                if status == 1 { 
                    // Assign the job to the selected node
                    let job_status = match client.start_job(job_request).await{
                        Ok(response) => response,
                        Err(e) => {debug!("{}", e); JobStatus{ request_id: -1, status_code: -1}}
                    };
                    let status = job_status.status_code;
                    if status != 1 && status != 2 {
                        // Update the database for this assignment
                        let new_job = NewJob{ request_id: &job_request.request_id, node_id: &random_node.id, 
                                                    completion_date: if status == 0 { None } else { Some(Utc::now().naive_utc()) }, 
                                                    status_code: &(status as i8), secure_hash: &String::from("")};
                        DatabaseHandler::insert_job(db_handler, &new_job).await?;
                        if status == 0 {
                            DatabaseHandler::update_node_working_state(db_handler, &random_node.id, true).await?;
                        }
                        else {
                            DatabaseHandler::finalize_job(db_handler, &job_request.request_id, "", &(status as i8)).await?;
                            DatabaseHandler::update_node_working_state(db_handler, &random_node.id, false).await?;
                        } 
                    }
                    retries = 0
                }
                else {
                    // Delay the execution of the loop
                    thread::sleep(Duration::from_secs(15));
                }
                // Halt the assignment attempts once the maximum number of retries have been reached
                if retries == 0 {
                    break;
                }
            }
        }

        Ok(true)
    }
}