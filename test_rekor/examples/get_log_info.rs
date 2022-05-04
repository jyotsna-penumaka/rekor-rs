use openapi::apis::{configuration::Configuration, tlog_api};
use openapi::models::LogInfo;

/* 
TO DO: Fix the bug
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Serde(Error("missing field `treeID`", line: 1, column: 339))',
examples/get_log_info.rs:7:75
*/

#[tokio::main]
async fn main() {
    let configuration = Configuration::default();
    let log_info : LogInfo = tlog_api::get_log_info(&configuration).await.unwrap();
    println!("{:#?}", log_info);
}
