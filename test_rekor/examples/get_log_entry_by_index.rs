use openapi::apis::{configuration::Configuration, entries_api};
use openapi::models::log_entry::LogEntry;

#[tokio::main]
async fn main() {
    let configuration = Configuration::default();
    let message: LogEntry = entries_api::get_log_entry_by_index(&configuration, 1)
        .await
        .unwrap();
    println!("{:#?}", message);
}
