use openapi::apis::{configuration::Configuration, timestamp_api};

#[tokio::main]
async fn main() {
    let configuration = Configuration::default();
    let cert_chain = timestamp_api::get_timestamp_cert_chain(&configuration).await;
    println!("{}", cert_chain.unwrap());
}
