use diesel::{prelude::*, sql_types::Char};
use serde::{Deserialize, Serialize};
use crate::schema::{cached_features, requests, jobs, joined_requests, joined_jobs, nodes, finalized_requests};
use anyhow::Result;

/* 

Here are the models for each table in the database. There are also structs that model the results of specific 
queries (e.g. QueriedJob) and were contructed manually, consisting of multiple joins.

*/

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Request {
    pub id: i64, 
    pub reference: String,
    pub candidates_list_id: i32,
    pub custom_list: String,
    pub uncached: String,
    pub hash_value: String,
    pub creation_date: chrono::NaiveDateTime,
    pub meta: bool,
    pub go_term: String,
    pub comparison_mode : i8,
    pub segment_start: i32,
    pub segment_end: i32,
    pub alignment_level : i8,
    pub views : i64,
}

impl Request {
    pub fn construct() -> Self {
        Self{ id: -1, reference: String::from(""), candidates_list_id: -1,  
            custom_list: String::from(""), uncached: String::from(""),   
            hash_value: String::from(""), creation_date: chrono::NaiveDateTime::MIN,
            meta: false, go_term: String::from(""), comparison_mode: -1, segment_start: -1,
            segment_end: -1, alignment_level: -1, views: -1}
    }
}


#[derive(QueryableByName, Debug, Serialize, Deserialize)]
#[diesel(table_name = finalized_requests)]
pub struct FinalizedRequest {
    pub id: i64, 
    pub reference: String,
    pub candidates_list_id: i32,
    pub custom_list: String,
    pub uncached: String,
    pub hash_value: String,
    pub creation_date: chrono::NaiveDateTime,
    pub meta: bool,
    pub go_term: String,
    pub comparison_mode : i8,
    pub segment_start: i32,
    pub segment_end: i32,
    pub alignment_level : i8,
    pub views : i64,
    pub secure_hash: String,
    pub list_name: Option<String>,
    pub status_code: i8,
}

impl FinalizedRequest {
    pub fn construct() -> Self {
        Self{ id: -1, reference: String::from(""), candidates_list_id: -1,  
            custom_list: String::from(""), uncached: String::from(""),   
            hash_value: String::from(""), creation_date: chrono::NaiveDateTime::MIN,
            meta: false, go_term: String::from(""), comparison_mode: -1, segment_start: -1,
            segment_end: -1, alignment_level: -1, views: -1, secure_hash: String::from(""),
            list_name: Some(String::from("")), status_code: -1
        }
    }
}


#[derive(QueryableByName,  Clone)]
#[diesel(table_name = joined_requests)]
pub struct QueriedRequest {
    pub id: i64, 
    pub reference: String,
    pub candidates_list_id: i32,
    pub custom_list: String,
    pub uncached: String,
    pub hash_value: String,
    pub creation_date: chrono::NaiveDateTime,
    pub meta: bool,
    pub go_term: String,
    pub comparison_mode : i8, 
    pub segment_start: i32,
    pub segment_end: i32,
    pub alignment_level: i8,
    pub views : i64,
    pub list_name: Option<String>,
}

impl QueriedRequest{

    pub fn construct() -> Result<Self> { 
        let date_time = chrono::NaiveDateTime::MIN;
        Ok(Self { id : -1,
            reference: String::from(""),
            candidates_list_id: -1,
            custom_list: String::from(""),
            uncached: String::from(""),
            hash_value: String::from(""),
            creation_date: date_time,
            meta: false,
            go_term: String::from(""),
            list_name: Some(String::from("")),
            comparison_mode: 0,
            segment_start: -1,
            segment_end: -1,
            alignment_level: -1,
            views: -1
          })
    }
}

#[derive(Queryable)]
pub struct Job {
    pub id: i64, 
    pub request_id: i64,
    pub node_id: i64,
    pub assignment_date: chrono::NaiveDateTime,
    pub completion_date: Option<chrono::NaiveDateTime>,
    pub last_checked_date: Option<chrono::NaiveDateTime>,
    pub status_code: i8,
    pub secure_hash: String,
}

#[derive(QueryableByName, Queryable)]
#[diesel(table_name = nodes)]
pub struct Node {
    pub id: i8, 
    pub ip: String,
    pub domain: String,
    pub active: bool,
    pub working: bool,
    pub sync_date: chrono::NaiveDateTime,
    pub cores: i8, 
}

impl Node {
    pub fn construct() -> Self {
        Self{ id: -1, ip: String::from(""), domain: String::from(""),active: false, 
              working: false, sync_date: chrono::NaiveDateTime::MIN, cores: -1}
    }
}

#[derive(Queryable)]
pub struct CachedFeatureId {
    pub id: i64, 
    pub structure_id: String,
} 

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct CandidateList{
    pub id: i32, 
    pub title: String,
}

#[derive(Insertable)]
#[diesel(table_name = cached_features)]
pub struct NewCachedFeatureId<'a> { 
    pub structure_id: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = jobs)]
pub struct NewJob<'a> { 
    pub request_id:&'a i64,
    pub node_id:&'a i8, 
    pub completion_date: Option<chrono::NaiveDateTime>,
    pub status_code:&'a i8,
    pub secure_hash: &'a str,
}
 
#[derive(Insertable)]
#[diesel(table_name = requests)]
pub struct NewRequest<'a> { 
    pub reference: &'a str,
    pub candidates_list_id:&'a i32,
    pub custom_list: &'a str,
    pub uncached: &'a str,
    pub hash_value: &'a str,
    pub meta: bool,
    pub go_term: &'a str,
    pub segment_start:&'a i32,
    pub segment_end:&'a i32,
    pub comparison_mode:&'a i8,
    pub alignment_level:&'a i8,
}

#[derive(QueryableByName,  Clone)]
#[diesel(table_name = joined_jobs)]
pub struct QueriedJob {
    pub id: i64,  
    pub hash_value: String,
    pub request_id: i64,  
    pub node_id: i8,
    pub node_ip: String,
    pub node_domain: String,
    pub comparison_mode: i8
}

impl QueriedJob{
    pub fn create_initialized() -> Result<Self> {                                                  
        Ok(Self { id : -1, 
            hash_value: String::from(""),
            request_id: -1,
            node_id: -1,
            node_ip: String::from(""), 
            node_domain: String::from(""),
            comparison_mode: -1
          })
    }
}


#[derive(QueryableByName)]
pub struct Uncached {
    #[diesel(sql_type = Char)]
    pub uncached: String,
}