use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write, Seek};
use std::path::{Path, self};
use std::{fs, env, io};
use anyhow::Result;
use glob::{MatchOptions, glob_with};
use unicode_segmentation::UnicodeSegmentation;
use zip::write::FileOptions;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher}; 
use tracing::debug;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use sha2::{Sha256, Digest};  
use walkdir::{DirEntry, WalkDir};
use regex::Regex;
use lazy_static::lazy_static;


/*

This module contains helper functions that are used in various modules
or functions that refer to very specific small tasks. 

*/

// Accumulate the filenames of the structure data files that are present in the specified file path
pub fn retrieve_cached_list(absolute_path: String) -> Result<Vec<String>>{
    let paths = fs::read_dir(absolute_path)?;
    let mut pdb_ids =  Vec::new();
    for path in paths 
    { 
        let filename = path?.path();
        let pdb_filename = filename.to_str();
        let mut pdb_id = match pdb_filename 
        {
            Some(id) => id,
            None => "",
        };
        // Split the filepath
        let mut parts: Vec<&str> = pdb_id.split(std::path::MAIN_SEPARATOR).collect();
        if parts.len() > 1 
        {
            parts = parts[parts.len() - 1].split(".").collect();
            if parts.len() > 1 
            {
                pdb_id = parts[0];
                if pdb_id.len() > 1 
                {
                    // Convert filename to uppercase, except for the part that is contained in AlphaFold filenames
                    let capitalized_id = pdb_id.to_string().to_uppercase().replace("-MODEL_V", "-model_v");
                    if !pdb_ids.contains(&capitalized_id)
                    {
                        pdb_ids.push(capitalized_id); 
                    }
                }
            }
        }
    }
    Ok(pdb_ids)
}

// Retrieve a string from JSON data
pub fn retrieve_json_str<'a>(data : &'a serde_json::Value, column_label: &str) -> &'a str{
    match data.get(column_label).and_then(|value| value.as_str())
    {
        None => {
                    debug!("Bad input: {}", column_label); 
                    ""
                },
        Some(entry) => entry
    }
}

// Retrieve an integer from JSON data
pub fn retrieve_json_int(data : &serde_json::Value, column_label: &str) -> i64 {
    match data.get(column_label).and_then(|value| value.as_i64())
    {
        None => {
                    debug!("Bad input: {}", column_label); 
                    -1
                },
        Some(number_input) => number_input.into()
    }
}

// Retrieve a boolean from JSON data
pub fn retrieve_json_bool(data : &serde_json::Value, column_label: &str) -> bool {
    match data.get(column_label).and_then(|value| value.as_bool())
    {
        None => {
                    debug!("Bad input: {}", column_label); 
                    false
                },
        Some(boolean_value) => boolean_value.into()
    }
}

// Get a substring from a UTF-8 string
pub fn get_substring(x : &String, max_characters: usize) -> String{
    let count = x.chars().count();
    if max_characters < count
    {
        let truncated : String = x.graphemes(true).take(max_characters).collect();
        return truncated;
    }
    x.clone()
}

// Compute a hash from an object
pub fn calculate_hash<T: Hash>(t: &T) -> String { 
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish().to_string()
}

pub fn check_structure_id(mut structure_id : &str) -> Result<&str> {
    // All input is changed to upper case 
    lazy_static! {
        static ref PDB_RE: Regex = Regex::new(r"[A-Z|0-9]{4}").unwrap();
        static ref AF_RE: Regex = Regex::new(r"AF-[A-Z|0-9]{3,}-F[0-9]*-MODEL_V4").unwrap();
        static ref ESM_RE: Regex = Regex::new(r"MGYP[0-9]{12}").unwrap();
    }
    if PDB_RE.is_match(structure_id) == false &&
       AF_RE.is_match(structure_id)  == false &&
       ESM_RE.is_match(structure_id)  == false {
        structure_id = "";
    }

    Ok(structure_id)
}

pub fn check_composite_id(id : &str) -> Result<Vec<String>> { 
    let mut id_parts: Vec<String> = id.split("_").into_iter().map(|s| s.trim().to_string().to_uppercase()).collect();

    // Retrieve the parts of the id
    let mut structureid = match id_parts.get(0) {
        Some(element) => element.to_string(),
        None => String::from("")
    };

    let mut chainid = match id_parts.get(1) {
        Some(element) => element.to_string(),
        None => String::from("")
    };

    // Determine whether there is garbage input
   structureid = check_structure_id(&structureid)?.to_string();

    if chainid != "" && chainid.chars().all(char::is_alphanumeric) == false {
        chainid = String::from("");
    }

    if structureid == "" || chainid == "" {
        id_parts = Vec::<String>::new();
    }

    Ok(id_parts)
}

pub fn store_cached_structures(path: String)  -> Result<()> {
    use crate::db::dbhandler::DatabaseHandler;
    use futures::executor::block_on;
    let mut db_handler = DatabaseHandler::new()?;
    let filelist = retrieve_cached_list(path)?;
    block_on(DatabaseHandler::insert_structure_ids(&mut db_handler, &filelist ))
}

#[derive(Debug, Serialize, Deserialize)]
struct CaptchaResponse {
    pub success: bool,
    #[serde(rename="error-codes")]
    pub error_codes: Option<Vec<String>>
}

pub async fn verify_captcha_token(token: String) -> Result<bool>{ 
    dotenv().ok(); 
    let captcha_secret = env::var("CAPTCHA_SECRET")?; 
    let params = [("secret", &captcha_secret), ("response", &token)];
    let client = Client::new();
    let response = client.post("https://www.google.com/recaptcha/api/siteverify")
            .form(&params)
            .send().await?.text().await?;
    let captcha_result: CaptchaResponse = serde_json::from_str(&response)?;
    match captcha_result.error_codes {
        Some(codes) => debug!("reCaptcha error: {:?}", codes ),
        None => debug!("No error codes")
    }
    Ok(captcha_result.success)
}

pub fn compute_file_hash(path : &str) -> Result<String> {
    let mut file = fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let _n = io::copy(&mut file, &mut hasher)?;
    let hash = hasher.finalize(); 
    Ok(format!("{:x}", hash))
}

// The following function is retrieved from: 
// https://github.com/zip-rs/zip/blob/e32db515a2a4c7d04b0bf5851912a399a4cbff68/examples/write_dir.rs
fn zip_dir<T>(
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: &str,
    writer: T
) -> Result<()>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix))?;

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            // println!("adding file {path:?} as {name:?} ...");
            #[allow(deprecated)]
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            // println!("adding dir {path:?} as {name:?} ...");
            #[allow(deprecated)]
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Ok(())
}

// Compress a directory
pub fn compress_directory(source_directory: &str, output_path: &str) -> Result<bool>{
    let mut result = true;
    if Path::new(&source_directory).exists() == true {
        let archive = File::create(output_path)?;
        let walkdir = WalkDir::new(source_directory);
        let it = walkdir.into_iter();
        match zip_dir(&mut it.filter_map(|e| e.ok()), source_directory, archive){
            Ok(_) => { result = true; },
            Err(e) => { debug!("{}", e); result = false; } 
        }
    }
    Ok(result)
}

// Extract files with a specific extension from an archive
pub fn extract_result_files(archive_path: &str, output_path: &str, extension: &str) -> Result<()> {
    let file_name = std::path::Path::new(archive_path);
    let zipfile = std::fs::File::open(file_name)?;

    let mut archive_file = zip::ZipArchive::new(zipfile)?;

    for i in 0..archive_file.len() {
        let mut compressed_file = archive_file.by_index(i)?;
        let internal_path = match compressed_file.enclosed_name() {
            Some(path) => path,
            None => {
                continue;
            }
        };
        let file_extension = match internal_path.extension() {
            Some(name) => match name.to_str(){
                Some(fname) => fname,
                None => ""
            },
            None => "",
        }; 
        if file_extension == extension {
            let file_name = match internal_path.file_name() {
                Some(name) => match name.to_str(){
                    Some(fname) => fname,
                    None => ""
                },
                None => "",
            }; 
            if file_name.len() > 0 && file_extension.len() > 0 {
                let mut content = Vec::new();
                let mut output_file = File::create([output_path, 
                                                        &std::path::MAIN_SEPARATOR.to_string(), 
                                                        file_name].join(""))?;
                compressed_file.read_to_end(&mut content)?;
                output_file.write_all(&content)?;
            }
        }
    }

    Ok(())
}

// Retrieve html filenames for the quick view of the results of a request
pub fn get_html_filenames(hash_value : &str, meta : &bool, go_term: &str, root_path: &str) -> Result<HashMap<String, Vec<String>>>
{
    let options = MatchOptions {
        case_sensitive: true,
        require_literal_separator: false,
        require_literal_leading_dot: false
    };
    // The function is looking for filenames with specific suffices and extensions
    let name_suffixes = ["-merged-notenriched_report.html",
                                    "-merged-enriched_eval_report.html",
                                    "-merged-h-enriched_report.html",
                                    "-pres_report.html"];
    let mut html_file_names = HashMap::new();
    if Path::new(&root_path).exists() {
        for entry in glob_with(&[root_path, &path::MAIN_SEPARATOR.to_string(),  
                                                            hash_value, &path::MAIN_SEPARATOR.to_string(), 
                                                            "*.html"].join(""), options)? {
            if let Ok(path) = entry {
                let filename = match path.file_name() {
                    Some(file_name) => match file_name.to_str(){
                        Some(name) => name,
                        None => ""
                    },
                    None => ""
                };

                // Each html filename is mapped to a label that will be parsed in the UI interface of MachaonWeb
                if filename.len() > 0{
                    if filename.contains(name_suffixes[0]){
                        html_file_names.entry(String::from("cluster")).or_insert(Vec::new()).push(filename.to_string());
                    }
                    if *meta == false {
                        continue
                    }
                    if filename.contains(name_suffixes[1]){ 
                        html_file_names.entry(String::from("top")).or_insert(Vec::new()).push(filename.to_string());
                    }
                    if filename.contains(name_suffixes[2]){ 
                        html_file_names.entry(String::from("topHuman")).or_insert(Vec::new()).push(filename.to_string());
                    }
                    if go_term.len() == 0{
                        continue
                    }
                    if filename.contains(name_suffixes[3]) && filename.contains(go_term) { 
                        html_file_names.entry(String::from("goTerm")).or_insert(Vec::new()).push(filename.to_string());
                    }
                }
            }
        }
    }

    Ok(html_file_names)
}