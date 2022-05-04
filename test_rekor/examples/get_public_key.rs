use openapi::apis::{configuration::Configuration, pubkey_api};

#[tokio::main]
async fn main() {
    let configuration = Configuration::default();
    let pubkey = pubkey_api::get_public_key(&configuration, None).await;
    println!("{:#?}", pubkey.unwrap());
}
