use diesel::{prelude::*, sql_types::{BigInt, Bool, Integer, Char, TinyInt}, dsl::{sql, count}, sql_query}; //, debug_query};
use dotenvy::dotenv;
use rand::Rng;
use std::env;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    MysqlConnection,
};
use anyhow::Result;
use crate::{db::models::{NewCachedFeatureId, CachedFeatureId}, schema::{requests::{self}, jobs::{self}, nodes::{self, working}, cached_features, candidate_lists}};

use super::models::{NewRequest, CandidateList, NewJob, Node, QueriedRequest, QueriedJob, FinalizedRequest, Uncached};


// This is the MariaDB access layer of MachaonWeb

#[derive(Debug)]
pub struct DatabaseHandler{ 
    connection_pool: Pool<ConnectionManager<MysqlConnection>>,
}

impl DatabaseHandler {

    // Create a new struct instance
    pub fn new() -> Result<Self> {
        dotenv().ok(); 
        let database_url = env::var("DATABASE_URL")?; 
        let db_manager = ConnectionManager::<MysqlConnection>::new(&database_url);
        let db_pool = Pool::builder().test_on_check_out(true).build(db_manager)?; 
        Ok(Self { connection_pool : db_pool })
    }
 
    // Update the table of processed structure IDs with a new one
    pub async fn insert_structure_ids(&mut self, data: &Vec<String>) -> Result<()> {
        let insert_values: Vec<NewCachedFeatureId> = data.into_iter().map(|entry| NewCachedFeatureId {structure_id : entry}).collect();
        let db_connection = &mut self.connection_pool.get()?;

        diesel::insert_into(cached_features::table)
            .values(&insert_values)
            .execute(db_connection)?;
        Ok(())
    }

    // Retrieve all the structure IDs which have been preprocessed
    pub async fn get_cached_pbd_ids(&self) -> Result<Vec<CachedFeatureId>> {
        let db_connection = &mut self.connection_pool.get()?;
        let results: Vec<CachedFeatureId> = cached_features::table.load::<CachedFeatureId>(db_connection)?;
        Ok(results)
    }

    // Retrieve the available preset candidate lists 
    pub async fn get_candidate_lists(&self) -> Result<Vec<CandidateList>> {
        let db_connection = &mut self.connection_pool.get()?;
        let results: Vec<CandidateList> = candidate_lists::table.load::<CandidateList>(db_connection)?;
        Ok(results)
    }

    // Retrieve how many nodes are currently designated as active
    pub async fn get_active_node_count(&self) -> Result<Option<i64>> {
        let db_connection = &mut self.connection_pool.get()?;
        let result = nodes::table.select(count(nodes::id))
                                      .filter(nodes::active.eq(true)).load::<i64>(db_connection)?;
        Ok(result.into_iter().next())
    }

    // Retrieve information on the nodes that are currently designated as active
    pub async fn get_available_nodes(&self) -> Result<Vec<Node>> {
        let db_connection = &mut self.connection_pool.get()?;
        let result: Vec<Node> = nodes::table.filter(nodes::working.eq(false).and(nodes::active.eq(true)))
                                            .order(nodes::sync_date.asc()).load::<Node>(db_connection)?;
        Ok(result)
    }

    // Retrieve how many jobs are currently being executed
    pub async fn get_running_jobs_count(&self) -> Result<Option<i64>> {
        let db_connection = &mut self.connection_pool.get()?;
        let result = requests::table.select(count(requests::id))
                                    .inner_join(jobs::table.on(requests::id.eq(jobs::request_id)))
                                    .filter(jobs::completion_date.is_null().and(jobs::status_code.eq(0))).load::<i64>(db_connection)?;
        Ok(result.into_iter().next())
    }

    // Retrieve how many requests are currently being queued
    pub async fn get_queued_requests_count(&self) -> Result<Option<i64>> {
        let db_connection = &mut self.connection_pool.get()?;
        let result = requests::table.select(count(requests::id))
                                    .left_join(jobs::table.on(requests::id.eq(jobs::request_id)))
                                    .filter(jobs::id.is_null()).load::<i64>(db_connection)?;
        Ok(result.into_iter().next())
    }

    // Check if a request has a finished job. A finished job has a registered SHA-256 file hash.
    pub async fn get_fullfilled_request(&self, hash: &str, meta: &bool, go_term: &str) -> Result<Option<String>> {
        let db_connection = &mut self.connection_pool.get()?;
        let result = jobs::table.select(jobs::secure_hash)
                                    .left_join(requests::table.on(jobs::request_id.eq(requests::id)))
                                    .filter(jobs::status_code.eq(0)
                                    .and(requests::hash_value.eq(&hash))
                                    .and(requests::meta.eq(meta))
                                    .and(requests::go_term.eq(go_term)))
                                    .order(requests::id.desc())
                                    .limit(1)
                                    .load::<String>(db_connection)?;
        Ok(result.into_iter().next())
    }

    // Get the row id of a request using its hash
    pub async fn get_request_by_hash(&self, hash_code : &str) -> Result<Option<i64>> {
        let db_connection = &mut self.connection_pool.get()?;
        let result = requests::table.select(requests::id)
                                    .filter(requests::hash_value.eq(hash_code))
                                    .order(requests::id.desc())
                                    .limit(1)
                                    .load::<i64>(db_connection)?;
        Ok(result.into_iter().next())
    }

    // Retrieve information of a request using its id and hash
    pub async fn get_request_with_proof(&self, request_id: &i64, hash_code : &str) -> Result<Option<FinalizedRequest>> {
        let db_connection = &mut self.connection_pool.get()?;
        let result = sql_query("SELECT r.*, 
                                                        j.secure_hash AS secure_hash, 
                                                        cl.title As list_name,
                                                        j.status_code AS status_code
                                                        FROM requests AS r
                                                        INNER JOIN jobs as j 
                                                        ON r.id = j.request_id 
                                                        LEFT JOIN candidate_lists as cl 
                                                        ON r.candidates_list_id  = cl.id
                                                        WHERE r.id = ?
                                                        AND r.hash_value = ?
                                                        ORDER BY j.completion_date DESC 
                                                        LIMIT 1;")
                                                        .bind::<BigInt, _>(request_id) 
                                                        .bind::<Char, _>(hash_code) 
                                                        .get_results(db_connection)?;
        Ok(result.into_iter().next())
    }

    // Retrieve how many nodes are currently not executing a job
    pub async fn get_available_node_count(&self) -> Result<Option<i64>> {
        let db_connection = &mut self.connection_pool.get()?;
        let result = nodes::table.select(count(nodes::id))
                                    .filter(nodes::active.eq(true).and(nodes::working.eq(false))).load::<i64>(db_connection)?;
        Ok(result.into_iter().next())
    }

    // Create a new request row
    pub async fn insert_request(&self, data : &NewRequest<'_>) -> Result<()> {
        let db_connection = &mut self.connection_pool.get()?; 

        diesel::insert_into(requests::table)
            .values(data)
            .execute(db_connection)?;
        Ok(())
    }

    // Create a new job row
    pub async fn insert_job(&self, data : &NewJob<'_>) -> Result<()> {
        let db_connection = &mut self.connection_pool.get()?; 

        diesel::insert_into(jobs::table)
            .values(data)
            .execute(db_connection)?;
        Ok(())
    }

    // Update the views of a request
    pub async fn update_view_count(&self, request_id : &i64) -> Result<()> {
        let db_connection = &mut self.connection_pool.get()?; 
        diesel::update(requests::table).set(requests::views.eq(requests::views + 1))
            .filter(requests::id.eq(request_id))
            .execute(db_connection)?; 
        Ok(())
    }

    // Identify which of the provided structure IDs are not cached
    pub async fn get_uncached_structure_ids(&self, data: &Vec<String>) -> Result<Vec<String>> { 
        let db_connection = &mut self.connection_pool.get()?;
        let query = cached_features::table.select(cached_features::structure_id)
                                                        .filter(cached_features::structure_id.eq_any(data));
        let results: Vec<String> = query.load::<String>(db_connection)?;
        let uncached_ids: Vec<String> =  data.into_iter().filter(|&item| !results.contains(&item)).cloned().collect();
        Ok(uncached_ids)
    }

    // Update the availability of a node
    pub async fn update_node_working_state(&self, node_id : &i8, status :  bool) -> Result<()> {
        let db_connection = &mut self.connection_pool.get()?; 
        diesel::update(nodes::table)
            .filter(crate::schema::nodes::id.eq(node_id))
            .set(working.eq(status))
            .execute(db_connection)?;
        Ok(())
    }

    // Retrieve the id of a request made recently during the specified timeframe
    pub async fn check_early_request(&self) -> Result<usize> { 
        let minute_interval = rand::thread_rng().gen_range(2..5);
        let db_connection = &mut self.connection_pool.get()?;
        let result: Vec<i64> = requests::table.select(requests::id)
                                            .filter(sql::<Bool>("TIMESTAMPDIFF(MINUTE,  creation_date, NOW()) <= ")
                                            .bind::<Integer,_>(minute_interval))
                                            .limit(1)
                                            .load(db_connection)?;
        Ok(result.len())
    }

    // Retrieve a recent pending request
    pub async fn get_early_pending_request(&self) -> Result<Vec<QueriedRequest>> { 
        let db_connection = &mut self.connection_pool.get()?;
        let entries = sql_query("SELECT r.*, c.title AS list_name FROM requests as r
                                            INNER JOIN
                                            (SELECT MIN(r.id) as id
                                            FROM requests as r
                                            LEFT JOIN jobs as j
                                            ON r.id = j.request_id
                                            WHERE j.id IS NULL) AS j
                                            ON r.id = j.id
                                            LEFT JOIN candidate_lists AS c
                                            ON r.candidates_list_id = c.id 
                                            LIMIT 1")
                                            .get_results(db_connection)?; 
        Ok(entries)
    }

    // Get the name of a candidate list by using its ID
    pub async fn get_candidate_list_name(&self, list_id: &i32) -> Result<String> {
        let db_connection = &mut self.connection_pool.get()?;
        let result: String = candidate_lists::table.select(candidate_lists::title)
                                            .filter(candidate_lists::id.eq(list_id))
                                            .first(db_connection)?;
        Ok(result)
    }

    // Verify that the provided candidate list ID exists
    pub async fn verify_candidate_list(&self, list_id: &i32) -> Result<i32> {
        let db_connection = &mut self.connection_pool.get()?;
        let result = candidate_lists::table.select(candidate_lists::id)
                                                          .filter(candidate_lists::id.eq(list_id))
                                                          .first::<i32>(db_connection)?;
        Ok(result)
    }

    // Update the status of a job
    pub async fn update_job_status(&self, job_id: &i64, status: &i8) -> Result<()> {
        let db_connection = &mut self.connection_pool.get()?; 

        diesel::update(jobs::table)
            .filter(crate::schema::jobs::id.eq(job_id))
            .set(jobs::status_code.eq(status))
            .execute(db_connection)?;
        Ok(())
    }

    // Store the timestamp of the last check on a job
    pub async fn update_job_check(&self, job_id : &i64) -> Result<()> {
        let db_connection = &mut self.connection_pool.get()?; 
        let query = diesel::sql_query("UPDATE jobs SET last_checked = NOW() WHERE id = ?")
        .bind::<BigInt, _>(job_id);
        query.execute(db_connection)?; 
        Ok(())
    }

    // Store the timestamp of the last sync of a node in MachaonWeb network
    pub async fn update_node_sync_date(&self, node_id : &i8) -> Result<()> {
        let db_connection = &mut self.connection_pool.get()?; 

        diesel::sql_query("UPDATE nodes SET sync_date = NOW() WHERE id = ?")
        .bind::<TinyInt, _>(node_id)
        .execute(db_connection)?; 
        Ok(())
    }

    // Update certain columns of a job row that mark it as complete.
    // secure hash: SHA-256 filehash of the compressed file that contains the results
    pub async fn finalize_job(&self, job_id: &i64, hash: &str, status: &i8) -> Result<()> {
        let db_connection = &mut self.connection_pool.get()?; 

        diesel::sql_query("UPDATE jobs SET last_checked = NOW(), completion_date =NOW(),
                           secure_hash = ?, status_code = ? WHERE id = ?")
                            .bind::<Char, _>(hash)
                            .bind::<TinyInt, _>(status)
                            .bind::<BigInt, _>(job_id) 
                            .execute(db_connection)?;  
        Ok(())
    }

    // Retrieve information of a currentl running job by random selection
    pub async fn get_running_job(&self) -> Result<Option<QueriedJob>> {
        let db_connection = &mut self.connection_pool.get()?;
        let entry = sql_query("SELECT j.id AS id, 
                                                r.hash_value AS hash_value,
                                                r.id AS request_id,
                                                n.id AS node_id, 
                                                n.ip AS node_ip,
                                                n.domain AS node_domain,
                                                r.comparison_mode AS comparison_mode
                                                FROM jobs as j
                                                INNER JOIN
                                                (SELECT MIN(rr.id) as id
                                                FROM requests as rr
                                                INNER JOIN jobs as jj
                                                ON rr.id = jj.request_id
                                                WHERE jj.completion_date IS NULL
                                                AND jj.status_code = 0) AS m
                                                ON m.id = j.request_id
                                                INNER JOIN requests AS r
                                                ON r.id = m.id
                                                INNER JOIN nodes AS n
                                                ON j.node_id = n.id
                                                ORDER BY RAND()
                                                LIMIT 1;")
                                        .load::<QueriedJob>(db_connection)?; 
        Ok(entry.into_iter().next())
    }

    // Retrieve information of a computing node in the MachaonWeb network that has outdated data
    pub async fn get_outdated_node(&self) -> Result<Option<Node>> {
        let db_connection = &mut self.connection_pool.get()?;
        let query = sql_query("SELECT n.*
                                    FROM nodes as n
                                    INNER JOIN
                                    (SELECT MIN(sync_date) as last_synced
                                    FROM nodes) AS l
                                    ON n.sync_date = l.last_synced
                                    WHERE active = 1
                                    AND working = 0
                                    ORDER BY cores
                                    LIMIT 1;");
        // println!("{}", debug_query::<diesel::mysql::Mysql, _>(&query));
        let entry = query.load::<Node>(db_connection)?; 
        Ok(entry.into_iter().next())
    } 

    // Retrieve the uncached structure IDs for each request that were made in a specified time interval
    pub async fn get_uncached_by_date(&self, node_id: i8, target_date: chrono::NaiveDateTime) -> Result<Vec<Uncached>> { 
        let db_connection = &mut self.connection_pool.get()?;
        let query = sql_query("SELECT rr.uncached 
                                            FROM requests as rr
                                            INNER JOIN jobs as jj
                                            ON rr.id = jj.request_id
                                            INNER JOIN nodes AS n
                                            ON n.id = jj.node_id  
                                            AND n.id <> ?
                                            WHERE rr.uncached <> ''
                                            AND jj.assignment_date > ?
                                            AND status_code = 0;")
                                            .bind::<TinyInt, _>(node_id) 
                                            .bind::<diesel::sql_types::Datetime,_>(target_date);
        // println!("{}", debug_query::<diesel::mysql::Mysql, _>(&query));
        let entry = query.load::<Uncached>(db_connection)?; 
        Ok(entry)
    }



}