use openapi::apis::{configuration::Configuration, server_api};

#[tokio::main]
async fn main() {
    let configuration = Configuration::default();
    let rekor_version = server_api::get_rekor_version(&configuration).await;
    println!("{:#?}", rekor_version.unwrap());
}
