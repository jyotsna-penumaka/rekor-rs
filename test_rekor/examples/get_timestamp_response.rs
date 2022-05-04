use openapi::apis::{configuration::Configuration, timestamp_api};
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    let configuration = Configuration::default();
    let request_path = PathBuf::from(r"test_request.tsq");
    let response_path = timestamp_api::get_timestamp_response(&configuration, request_path)
        .await
        .unwrap();
    println!("Rekor's response was saved in: {:#?}", response_path);
}
