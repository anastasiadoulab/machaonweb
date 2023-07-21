use std::env;

use crate::{rest::AppServer, monitor::Monitor};
use anyhow::Result;
use tracing::debug; 

pub mod db;
pub mod utils;
pub mod schema;
pub mod rest;
pub mod grpc;
pub mod logic;
pub mod monitor;
use dotenvy::dotenv;

pub mod jobreceiver {
    tonic::include_proto!("jobreceiver");
}

#[tokio::main]
async fn main() -> Result<()> { 
    dotenv().ok(); 

    // Tracing
    let file_appender = tracing_appender::rolling::daily(env::var("LOG_DIR")?, "axum.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
    .with_writer(non_blocking)
    .with_max_level(tracing::Level::DEBUG)
    .init();

    // Configurations
    let monitor_path: &'static str = Box::leak(env::var("MONITOR_PATH")?.to_string().into_boxed_str());
    let output_path: &'static str = Box::leak(env::var("OUTPUT_PATH")?.to_string().into_boxed_str());
    let web_protocol = "https://"; // "http://";
    let request_monitoring_enabled = true;
    let job_monitoring_enabled = true;
    let node_synchronization_enabled = true;

    // Request monitoring loop start
    if request_monitoring_enabled == true {
        tokio::spawn( async move{
            match Monitor::new(monitor_path, output_path, web_protocol){
                Ok(mut module) => {
                    println!("Monitoring requests started."); 
                    module.monitor_requests().await; true
                },
                Err(e) => {
                    debug!("Request monitoring failed: {}", e); 
                    false
                }
            };
        });
    }

    // Job monitoring loop start
    if job_monitoring_enabled == true {
        tokio::spawn( async move{
            match Monitor::new(monitor_path, output_path, web_protocol){
                Ok(mut module) => {
                    println!("Job monitoring started."); 
                    module.monitor_jobs().await; true
                },
                Err(e) => {
                    debug!("Job monitoring failed: {}", e); 
                    false
                }
            };
        });
    }

    // Node synchronization loop start
    if node_synchronization_enabled == true {
        tokio::spawn( async move{
            match Monitor::new(monitor_path, output_path, web_protocol){
                Ok(mut module) => {
                    println!("Node synchronization enabled."); 
                    module.sync_nodes().await; true
                },
                Err(e) => {
                    debug!("Node synchronization failed: {}", e); 
                    false
                }
            };
        });
    }

    // Server
    println!("MachaonWeb is starting."); 
    AppServer::new(env::var("WEB_SERVER_IP")?, env::var("WEB_SERVER_PORT")?.parse::<u16>()?, output_path.to_string())?.start().await?;

    Ok(())
}


