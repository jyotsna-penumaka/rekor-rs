/*
 * Rekor
 *
 * Rekor is a cryptographically secure, immutable transparency log for signed software releases.
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};
use crate::models::log_entry::LogEntry;
use serde_json::{Deserializer, Value}; 

/// struct for typed errors of method [`create_log_entry`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLogEntryError {
    Status400(crate::models::Error),
    Status409(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_log_entry_by_index`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLogEntryByIndexError {
    Status404(),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_log_entry_by_uuid`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLogEntryByUuidError {
    Status404(),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`search_log_query`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchLogQueryError {
    Status400(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogEntries {
    entries: Vec<LogEntry>
}

/// Creates an entry in the transparency log for a detached signature, public key, and content. Items can be included in the request or fetched by the server when URLs are specified. 
pub async fn create_log_entry(configuration: &configuration::Configuration, proposed_entry: crate::models::ProposedEntry) -> Result<LogEntry, Error<CreateLogEntryError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/log/entries", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&proposed_entry);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let uuid: &str = &local_var_content[1..67];
        let rest: &str = &local_var_content[69..local_var_content.len() - 2];
        let sum = "{\"uuid\": ".to_string() + &(uuid.to_owned()) + "," + rest;
        serde_json::from_str::<LogEntry>(&sum).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateLogEntryError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_log_entry_by_index(configuration: &configuration::Configuration, log_index: i32) -> Result<LogEntry, Error<GetLogEntryByIndexError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/log/entries", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("logIndex", &log_index.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let uuid: &str = &local_var_content[1..67];
        let rest: &str = &local_var_content[69..local_var_content.len() - 2];
        let sum = "{\"uuid\": ".to_string() + &(uuid.to_owned()) + "," + rest;
        serde_json::from_str::<LogEntry>(&sum).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetLogEntryByIndexError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns the entry, root hash, tree size, and a list of hashes that can be used to calculate proof of an entry being included in the transparency log
pub async fn get_log_entry_by_uuid(configuration: &configuration::Configuration, entry_uuid: &str) -> Result<LogEntry, Error<GetLogEntryByUuidError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/log/entries/{entryUUID}", local_var_configuration.base_path, entryUUID=crate::apis::urlencode(entry_uuid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let uuid: &str = &local_var_content[1..67];
        let rest: &str = &local_var_content[69..local_var_content.len() - 2];
        let sum = "{\"uuid\": ".to_string() + &(uuid.to_owned()) + "," + rest;
        serde_json::from_str::<LogEntry>(&sum).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetLogEntryByUuidError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn search_log_query(configuration: &configuration::Configuration, entry: crate::models::SearchLogQuery) -> Result<LogEntries, Error<SearchLogQueryError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/log/entries/retrieve", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&entry);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;
    println!("{:#?}", local_var_content);


    let stream = Deserializer::from_str(&local_var_content).into_iter::<Value>();

    println!("Printing the values: ");
    for value in stream {
        println!("hello");
        println!("{}", value.unwrap());
    }

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let uuid: &str = &local_var_content[1..67];
        let rest: &str = &local_var_content[69..local_var_content.len() - 2];
        let sum = "{\"uuid\": ".to_string() + &(uuid.to_owned()) + "," + rest;
        serde_json::from_str::<LogEntries>(&sum).map_err(Error::from)
        //serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SearchLogQueryError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}


