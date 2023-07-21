use axum::{http::{HeaderValue, Method}, response::IntoResponse, routing::{get, post}, Router, extract::State, http::{StatusCode, self}};
use std::{net::SocketAddr, sync::Arc,  path::PathBuf, collections::HashMap, env};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::{db::{dbhandler::DatabaseHandler, models::{CandidateList, FinalizedRequest}}, logic::{self, RequestResponse, Info, RequestResult}};
use tracing::{debug, Instrument};
use axum_server::tls_rustls::RustlsConfig; 
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
    cors::CorsLayer
};
use dotenvy::dotenv;

/*

This module coordinates the endpoints of MachaonWeb's REST API.

*/

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    http_port: u16,
    https_port: u16,
}

#[derive(Debug)]
pub struct AppServer{
    ip: [u8; 4],
    https_port: u16,
    output_path: String,
    db_handler:  DatabaseHandler,
}


impl AppServer{
 
    // Create a new instance
    pub fn new(ip_string: String, https_port : u16, output_path: String) -> Result<Self> {
        let db_handler = DatabaseHandler::new()?;
        let collected_ip = ip_string.split(".")
                            .filter_map(|s| s.parse::<u8>().ok())
                            .collect::<Vec<u8>>();
        let mut ip: [u8; 4] = [0; 4];
        ip.copy_from_slice(&collected_ip[..4]);
        Ok(Self { ip, https_port, output_path, db_handler })
    }

    pub async fn start(self) -> Result<bool> {
        dotenv().ok(); 
        // SSL certificates configuration
        let ssl_config = RustlsConfig::from_pem_file(
            PathBuf::from(env::var("SSL_CERTS_PATH")?)
                .join("machaonweb.com.crt"),
            PathBuf::from(env::var("SSL_CERTS_PATH")?)
                .join("myserver.key"),
        ).await?; 

        let frontend_path = PathBuf::from(env::var("FRONTEND_PATH")?);
        let serve_dir = ServeDir::new(&frontend_path)
                    .not_found_service(ServeFile::new(&frontend_path.join("index.html")));

        let app_socket = SocketAddr::from((self.ip, self.https_port));
        let router = Router::new()
            //static file serving
            .nest_service("/", serve_dir.clone())
            .fallback_service(serve_dir)
            // endpoint routing 
            .route("/resultdata/:hash/:req_id",get(Self::fetch_result))
            .route("/request", post(Self::receive_request))
            .route("/info", get(Self::get_info))
            .route("/lists", get(Self::get_candidate_lists))
            // Cross-Origin access configuration (debugging)
            .layer(CorsLayer::new()
                    .allow_origin(env::var("CORS_URL1")?.parse::<HeaderValue>().unwrap())
                    .allow_methods([Method::POST])
                    .allow_headers([http::header::CONTENT_TYPE]),
            )
            .layer(CorsLayer::new()
            .allow_origin(env::var("CORS_URL2")?.parse::<HeaderValue>().unwrap())
            .allow_methods([Method::POST])
            .allow_headers([http::header::CONTENT_TYPE]),
    )       
            .layer(TraceLayer::new_for_http())
            .with_state(Arc::new(self));

        // HTTPS only, use axum::Server::bind(&app_socket) for unencrypted HTTP
        axum_server::bind_rustls(app_socket, ssl_config)
        .serve(router.into_make_service())
        .await?;  
        Ok(true)
    }

    // Endpoint for retrieving the results of a request
    async fn fetch_result(state: State<Arc<AppServer>>,
        axum::extract::Path((hash, request_id)):
            axum::extract::Path<(String, i64)>
    ) -> impl IntoResponse {
        let app_server = state.0; 
        let response = match logic::get_request_result(&app_server.db_handler, &hash, &request_id, &app_server.output_path)
        .instrument(tracing::debug_span!("fetch_result")).await
        {
            Ok(result) => result,
            Err(e) => { debug!("Error: {}", e.to_string()); RequestResult{request: FinalizedRequest::construct(), files: HashMap::new()}}
        }; 
        (
            StatusCode::OK, 
            axum::Json(response)
        )
    }

    // Endpoint for receiving the request by a user
    async fn receive_request(state: State<Arc<AppServer>>,
        axum::extract::Json(data): axum::extract::Json<serde_json::Value>
    ) -> impl IntoResponse {
        let app_server = state.0; 
        let response = match logic::create_request(&app_server.db_handler, &data)
        .instrument(tracing::debug_span!("receive_request")).await
        {
            Ok(result) => result,
            Err(e) => { debug!("Error: {}", e.to_string()); RequestResponse::new(1, String::from(""), -1)}
        };
        (
            StatusCode::OK, 
            axum::Json(response)
        )
    }

    // Endpoint for MachaonWeb's status
    async fn get_info(state: State<Arc<AppServer>>) -> impl IntoResponse {

        let app_server = state.0;
        let response = match logic::retrieve_info(&app_server.db_handler)
        .instrument(tracing::debug_span!("get_info")).await
        {
            Ok(result) => result,
            Err(e) => { debug!("Error: {}", e.to_string()); Info::new(0, 0, 0)}
        };

        (
            StatusCode::OK, 
            axum::Json(response)
        )
    }

    // Endpoint for retrieving the available preset candidate lists
    async fn get_candidate_lists(state: State<Arc<AppServer>>) -> impl IntoResponse {

        let app_server = state.0;
        let response = match logic::get_candidate_lists(&app_server.db_handler)
        .instrument(tracing::debug_span!("get_candidate_lists")).await
        {
            Ok(result) => result,
            Err(e) => { debug!("Error: {}", e.to_string()); Vec::<CandidateList>::new()}
        };

        (
            StatusCode::OK, 
            axum::Json(response)
        )
    }

}

