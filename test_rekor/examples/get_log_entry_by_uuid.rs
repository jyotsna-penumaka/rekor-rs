use openapi::apis::{configuration::Configuration, entries_api};
use openapi::models::log_entry::LogEntry;

#[tokio::main]
async fn main() {
    let configuration = Configuration::default();
    let uuid = "073970a07c978b7a9ff15b69fe15d87dfb58fd5756086e3d1fb671c2d0bd95c0";
    let message: LogEntry = entries_api::get_log_entry_by_uuid(&configuration, &uuid)
        .await
        .unwrap();
    println!("{:#?}", message);
}
