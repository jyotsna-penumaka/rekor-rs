use openapi::apis::{configuration::Configuration, entries_api};
use openapi::models::{ProposedEntry};
use openapi::models::rekord::{Hash, Data, PublicKey, Signature, Spec};
use url::Url;

#[tokio::main]
async fn main() {
    let configuration = Configuration::default();

    let hash = Hash::new(
        "sha256".to_string(),
        "8b811ce3a8de7c778f64bdeabd0a30d9b3712c3d65d4324937471b1dfe6fc2a5".to_string(),
    );
    let data = Data::new(
        hash,
        Url::parse("https://raw.githubusercontent.com/jyotsna-penumaka/integrate-rekor/main/README.md").unwrap(),
    );
    let public_key = PublicKey::new(
        "c3NoLWVkMjU1MTkgQUFBQUMzTnphQzFsWkRJMU5URTVBQUFBSVA3M2tuT0tKYVNyVEtEa2U2OEgvRlJoODRZWU5CU0tBN1hPVWRpWmJjeG8gdGVzdEByZWtvci5kZXYK".to_string(),
    );
    let signature = Signature::new(
        "ssh".to_string(),
        "LS0tLS1CRUdJTiBTU0ggU0lHTkFUVVJFLS0tLS0KVTFOSVUwbEhBQUFBQVFBQUFETUFBQUFMYzNOb0xXVmtNalUxTVRrQUFBQWcvdmVTYzRvbHBLdE1vT1I3cndmOFZHSHpoaApnMEZJb0R0YzVSMkpsdHpHZ0FBQUFFWm1sc1pRQUFBQUFBQUFBR2MyaGhOVEV5QUFBQVV3QUFBQXR6YzJndFpXUXlOVFV4Ck9RQUFBRUNnVi8wbnZCTlQyRjZxUzJuWXpST1JmeVFWK1U2dzF1SmRSYUlpcHFGKzJuRGJKMkdvSWVGT1Mrakl5dlc0SVAKTEJkVUZ6cVhZcnZmaTJjeTM4aWFvSgotLS0tLUVORCBTU0ggU0lHTkFUVVJFLS0tLS0K".to_string(),
        public_key,
    );
    let spec = Spec::new(signature, data);

    let proposed_entry = 
        ProposedEntry::Rekord {
            api_version: "0.0.1".to_string(),
            spec: spec,
        };
        
    let message = entries_api::create_log_entry(&configuration, proposed_entry).await;
    println!("{:#?}", message);
}
