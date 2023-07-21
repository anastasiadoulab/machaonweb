use std::collections::HashMap;

use crate::db::models::FinalizedRequest;
use crate::{db::{dbhandler::DatabaseHandler, models::{NewRequest, CandidateList}}, utils};
use anyhow::Result; 
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument};


/*

This module is the middle layer between the REST API requests and the database.

*/


// Struct definitions that are used for structured responses (JSON)

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestResponse {
    status_code: i32,
    hash : String,
    request_id : i64
}

impl RequestResponse {

    pub fn new(status : i32, hash_code: String, request_id: i64) -> Self { 
        Self{ status_code: status, hash: hash_code, request_id: request_id}
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    nodes: i64,
    jobs: i64,
    queued: i64
}

impl Info {

    pub fn new(node_count : i64, job_count: i64, queue_count : i64) -> Self { 
        Self{  nodes: node_count, jobs: job_count, queued: queue_count }
    }

}


#[derive(Debug, Serialize, Deserialize)]
pub struct RequestResult {
    pub request: FinalizedRequest,
    pub files: HashMap<String, Vec<String>> 
}

impl RequestResult {

    pub fn new(fullfilled_request : FinalizedRequest, output_files: HashMap<String, Vec<String>>) -> Self { 
        Self{ request: fullfilled_request, files: output_files}
    }

}

// Functions that constitute the business logic of the web application


// Handle the job request
#[instrument(level="debug")]
pub async fn create_request(db_handler: &DatabaseHandler, data : &serde_json::Value) -> Result<RequestResponse>{
    let mut new_request: NewRequest = NewRequest { reference: "", custom_list: "", 
                                                    uncached: "", candidates_list_id: &-1, 
                                                    hash_value: "", go_term: "",  meta: false, 
                                                    segment_start: &-1, segment_end: &-1, comparison_mode: &0,
                                                    alignment_level: &-1 };
    // Parse the user input
    let candidate_list_id = utils::retrieve_json_int(&data, "candidateList") as i32;
    let custom_id_input = utils::get_substring(&utils::retrieve_json_str(&data, "customList")
                                    .to_string().trim().to_uppercase(), 5000).replace("-MODEL_V", "-model_v");
    let reference_input = utils::get_substring(&utils::retrieve_json_str(&data, "reference")
                                    .to_string().trim().to_uppercase(), 40).replace("-MODEL_V", "-model_v");
    let captcha_token = utils::retrieve_json_str(&data, "token");
    let mut uncached_string = String::from("");
    let mut custom_list_string = "";
    let comparison_mode = utils::retrieve_json_int(&data, "comparisonMode");
    let alignment_level = utils::retrieve_json_int(&data, "alignmentLevel");
    let mut segment_start = utils::retrieve_json_int(&data, "segmentStart");
    let mut segment_end = utils::retrieve_json_int(&data, "segmentEnd");
    let meta_analysis = utils::retrieve_json_bool(&data, "meta");
    let mut reference_id = String::from("");
    let ref_id_parts : Vec<String> = utils::check_composite_id(&reference_input)?; 
    let mut status = 0;
    let mut list_name_found = false;
    let mut structure_ids: Vec<String> = Vec::new();
    let mut hash_string = String::from("");
    let mut request_id = -1;


    let captcha_sucess = utils::verify_captcha_token(captcha_token.to_string()).await?;

    if captcha_sucess == false{ 
        status = -8;
    }

    //Check comparison mode option
    if status == 0 && (comparison_mode < 0 || comparison_mode > 2) {
        status = -9;
    }

    //Check alignment level option
    if status == 0 && (comparison_mode == 2 && (alignment_level < 0 || alignment_level > 3)) {
        status = -10;
    }

    //Process reference id
    if status == 0 {
        if ref_id_parts.len() > 0 && status == 0  {
            reference_id = ref_id_parts.join("_");
        } else { 
            status = -1;    
        }
    }
    
    //Check if the request is older than 10 minutes from the previous one
    if status == 0 {
        let early_request = match DatabaseHandler::check_early_request(&db_handler).await{
            Ok(amount) => amount > 0,
            Err(e) => { debug!("Early request : {}", e.to_string()); false }
        };   

        if early_request == true { 
            status = -2;
        }
    }

    //Check if id of candidate list exists
    if status == 0 && candidate_list_id > -1 {
        if comparison_mode == 2 { 
            status = -11;
        }
        else { 
            list_name_found = match DatabaseHandler::verify_candidate_list(&db_handler, &(candidate_list_id as i32)).await{
                Ok(entry) => { debug!("List id: {}", entry); entry >= 0 },
                Err(_e) => false
            };    

            if list_name_found == false { 
                status = -3;
            } 
            else {
                new_request.candidates_list_id = &candidate_list_id;
                // Check if the provided reference protein has not cached features
                let reference_slice = vec![ref_id_parts[0].clone()];
                let uncached_structure_ids = match DatabaseHandler::get_uncached_structure_ids(&db_handler, 
                                                          &reference_slice).await {
                    Ok(entries) => entries,
                    Err(e) => {debug!("{}", e); Vec::<String>::new()}
                };    

                if uncached_structure_ids.len() > 0 {
                    uncached_string = uncached_structure_ids.join(",");
                }
                
            }  
        }
    }

    // Process the custom provided candidate list 
    let custom_ids:Vec<String> = custom_id_input.split(",").into_iter()
                                .map(|s| s.to_string())
                                .collect();
    if status == 0 && list_name_found == false {
        if custom_ids.len() > 0  {   
            for id in &custom_ids {

                if id.len() > 0 {
                    if structure_ids.contains(id) == false {
                        structure_ids.push(id.clone());
                    }
                }
                else{
                    status = -4; 
                    break;     
                }
            } 

            if structure_ids.len() > 0 && status == 0 { 
                // Check if the provided reference protein or any protein in the candidate list has not cached features
                let reference_slice = vec![ref_id_parts[0].clone()];
                let uncached_structure_ids = match DatabaseHandler::get_uncached_structure_ids(&db_handler, 
                                                          &[&structure_ids[..], &reference_slice].concat()).await {
                    Ok(entries) => entries,
                    Err(e) => {debug!("{}", e); Vec::<String>::new()}
                };    

                if uncached_structure_ids.len() > 0 {
                    uncached_string = uncached_structure_ids.join(",");
                }
            }

        } else {
            status = -5;    
        } 

        if status == 0 && structure_ids.len() == 0 { 
            status = -6;
        }
    }

    // Check residue range for Segment comparison mode
    if comparison_mode == 2{
        if  segment_start > segment_end || 
            segment_end > 10000 || 
            segment_start > 10000 || 
            segment_end < 1 ||
            segment_start < 1 ||
            segment_end - segment_start > 600  ||
            segment_end - segment_start < 2
        { 
            status = -7;
        }
    }
    else
    {
        segment_start = -1;
        segment_end = -1;
    }

    // Prepare to store the new request in database
    let structure_ids_string = structure_ids.join(",");
    if status == 0 { 
        new_request.reference = reference_id.as_str();

        if uncached_string.len() > 0 { 
            new_request.uncached = uncached_string.as_str();
        }
        if structure_ids.len() > 0 { 
            custom_list_string = &structure_ids_string;
            new_request.custom_list = custom_list_string;
        }

        // Create hash of the request by its object
        let mut candidates = "";
        let list_id = candidate_list_id.to_string();
        if custom_list_string.len() > 0 { 
            candidates = custom_list_string;  
        }
        else if candidate_list_id > 0 { 
            candidates = &list_id; 
        }

        if candidates.len() > 0 { 
            hash_string = [reference_id.as_str(), candidates, &comparison_mode.to_string(), 
                           &segment_start.to_string(), &segment_end.to_string(), &alignment_level.to_string()].join("\n");
        }

        if hash_string.len() > 1 { 
            hash_string = utils::calculate_hash(&hash_string);
            new_request.hash_value = hash_string.as_str();
        
            let mode: i8 = comparison_mode as i8;
            let residue_start: i32 = segment_start as i32;
            let residue_end: i32 = segment_end as i32;
            new_request.meta = meta_analysis;
            new_request.segment_start = &residue_start;
            new_request.segment_end = &residue_end;
            new_request.comparison_mode = &mode;
            
            DatabaseHandler::insert_request(&db_handler, &new_request).await?;
            request_id = match DatabaseHandler::get_request_by_hash(&db_handler, &hash_string).await?{
                Some(req_id) => req_id,
                None => -1
            }
        }
        else
        {
            status = -11;
        }
    }  

    Ok(RequestResponse{status_code : status, hash: hash_string, request_id: request_id})
}

// Retrieve information of the MachaonWeb network's current status
#[instrument(level="debug")]
pub async fn retrieve_info(db_handler: &DatabaseHandler) -> Result<Info>{
 
    let nodes: i64 = match DatabaseHandler::get_active_node_count(&db_handler).await?{ 
        Some(count) => count,
        None => 0 
    };  

    let jobs = match DatabaseHandler::get_running_jobs_count(&db_handler).await?{ 
        Some(count) => count,
        None => 0
    };  

    let queued = match DatabaseHandler::get_queued_requests_count(&db_handler).await?{ 
        Some(count) => count,
        None => 0
    };  
    Ok(Info {nodes: nodes, jobs: jobs, queued: queued})
}

// Retrieve candidate lists
#[instrument(level="debug")]
pub async fn get_candidate_lists(db_handler: &DatabaseHandler) -> Result<Vec<CandidateList>>{
    let lists = match DatabaseHandler::get_candidate_lists(&db_handler).await {
        Ok(entries) => entries,
        Err(e) => {debug!("{}", e); Vec::<CandidateList>::new()}
    };    

    Ok(lists)
}

// Retrieve the result of a request
#[instrument(level="debug")]
pub async fn get_request_result(db_handler: &DatabaseHandler, hash: &str, request_id: &i64, root_path: &str) -> Result<RequestResult>{
 
    let request_row = match DatabaseHandler::get_request_with_proof(&db_handler, &request_id, &hash).await?{ 
        Some(row) => row,
        None => FinalizedRequest::construct() 
    };  
    let mut output_files = HashMap::new();

    if request_row.id > 0 {
        // Get the html filenames for quickview
        output_files = utils::get_html_filenames(&request_row.hash_value, &request_row.meta, 
                                                                        &request_row.go_term,
                                                                        root_path)?;
        // Update the view count of the request                                                                        
        DatabaseHandler::update_view_count(&db_handler, &request_id).await?;
    }
    Ok(RequestResult {request: request_row, files: output_files})
    
}