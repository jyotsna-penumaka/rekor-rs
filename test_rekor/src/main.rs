use openapi::apis::{configuration::Configuration, entries_api};
use openapi::models::{ProposedEntry, log_entry::LogEntry};
use openapi::models::rekord::{Hash, Data, PublicKey, Signature, Spec};
use url::Url;

#[tokio::main]
async fn main() {
    let configuration = Configuration::default();

    // Test create_log_entry 
    println!("Test#1 create_log_entry!");
    let hash = Hash::new(
        "sha256".to_string(),
        "0defde9b5c64102ed157d35ca0bdaf3208846484f759c75799f03f55f973fd5b".to_string(),
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
        "LS0tLS1CRUdJTiBTU0ggU0lHTkFUVVJFLS0tLS0KVTFOSVUwbEhBQUFBQVFBQUFETUFBQUFMYzNOb0xXVmtNalUxTVRrQUFBQWcvdmVTYzRvbHBLdE1vT1I3cndmOFZHSHpoaApnMEZJb0R0YzVSMkpsdHpHZ0FBQUFFWm1sc1pRQUFBQUFBQUFBR2MyaGhOVEV5QUFBQVV3QUFBQXR6YzJndFpXUXlOVFV4Ck9RQUFBRUNNaXJBNmNUMVUzRTUrM0pkWURhdTFVWG1ZcWRzMUxPMVppN04vYVh2a2puWW4yYmlFUzBYUTJ4R3FNOGFXQUYKUU4zTWcxYmxPL3B4UzQzcURER3pBUAotLS0tLUVORCBTU0ggU0lHTkFUVVJFLS0tLS0K".to_string(),
        public_key,
    );
    let spec = Spec::new(signature, data);

    let proposed_entry = 
        ProposedEntry::Rekord {
            api_version: "0.0.1".to_string(),
            spec: spec,
        };
        

/*     let uuid: &str = &response[1..67];
    let rest: &str = &response[69..response.len() - 2];
    let sum = "{\"uuid\": ".to_string() + &(uuid.to_owned()) + "," + rest;
    let v: Result<LogEntry, serde_json::Error> = serde_json::from_str(&sum);
    v.map_err(|err| Box::new(err) as Box<dyn std::error::Error>) */

    let message: LogEntry = entries_api::create_log_entry(&configuration, proposed_entry).await.unwrap();
    println!("{:#?}", message);

    // Test get_log_entry_by_index 
/*     println!("Test#2 get_log_entry_by_index");
    let message = entries_api::get_log_entry_by_index(&configuration, 1).await;
    println!("{:#?}", message); */


}
