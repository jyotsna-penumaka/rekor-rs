use openapi::apis::{configuration::Configuration, entries_api, index_api, pubkey_api, server_api, timestamp_api, tlog_api};
use openapi::models::{ProposedEntry, log_entry::LogEntry, SearchLogQuery, SearchIndex, search_index_public_key, LogInfo, ConsistencyProof};
use openapi::models::rekord::{Hash, Data, PublicKey, Signature, Spec};
use url::Url;
use std::path::PathBuf;

// Test the functionality of the Rekor Rust API

#[tokio::main]
async fn main() {
    let configuration = Configuration::default();

    let hash = Hash::new(
        "sha256".to_string(),
        "e2535d638859bb63ea9ea5cf467562cba63b007eae1acd0d73a3f259c582561f".to_string(),
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
        "LS0tLS1CRUdJTiBTU0ggU0lHTkFUVVJFLS0tLS0KVTFOSVUwbEhBQUFBQVFBQUFETUFBQUFMYzNOb0xXVmtNalUxTVRrQUFBQWcvdmVTYzRvbHBLdE1vT1I3cndmOFZHSHpoaApnMEZJb0R0YzVSMkpsdHpHZ0FBQUFFWm1sc1pRQUFBQUFBQUFBR2MyaGhOVEV5QUFBQVV3QUFBQXR6YzJndFpXUXlOVFV4Ck9RQUFBRURCYVFtVTRXNHFCQzBaM3N6aTZuSEE4cWlBdE5QVzFkU29UTmtlMTBOKzRRTUdad0pRMXR6QTVIYk5BUkxHc3cKN0I0b2RxWWFpRVEwSzMwdEtBZEcwSAotLS0tLUVORCBTU0ggU0lHTkFUVVJFLS0tLS0K".to_string(),
        public_key,
    );
    let spec = Spec::new(signature, data);

    let proposed_entry = 
        ProposedEntry::Rekord {
            api_version: "0.0.1".to_string(),
            spec: spec,
        };
    
    let proposed_entry_2 = proposed_entry.clone();

/*     // Test#1 create_log_entry 
    println!("____________________________________________________________________________");
    println!("____________________________________________________________________________");
    println!("Test#1 create_log_entry!");
    let log_entry: LogEntry = entries_api::create_log_entry(&configuration, proposed_entry).await.unwrap();
    println!("{:#?}", log_entry); */

    // Test#2 get_log_entry_by_index 
    println!("____________________________________________________________________________");
    println!("____________________________________________________________________________");
    println!("Test#2 get_log_entry_by_index");
    let message: LogEntry = entries_api::get_log_entry_by_index(&configuration, 1).await.unwrap();
    println!("{:#?}", message);

    // Test#3 get_log_entry_by_uuid 
    println!("____________________________________________________________________________");
    println!("____________________________________________________________________________");
    println!("Test#3 get_log_entry_by_uuid");
    let uuid = "073970a07c978b7a9ff15b69fe15d87dfb58fd5756086e3d1fb671c2d0bd95c0";
    let message: LogEntry = entries_api::get_log_entry_by_uuid(&configuration, &uuid).await.unwrap();
    println!("{:#?}", message);  

    // Test#4 search_log_query
    // Note: This needs some work. I was not able to parse the returned response into correct structs!
    println!("____________________________________________________________________________");
    println!("____________________________________________________________________________");
    println!("Test#4 search_log_query");
    let query = SearchLogQuery {
        entry_uuids: Some(vec!["3bb6b37275642a289650d81cd5ba8aba0adcf9b5a5330d21bf9dd6157f4cbb67".to_string()]),
        log_indexes: Some(vec![1594206]),
        entries: Some(vec![proposed_entry_2]),
    };
    let message = entries_api::search_log_query(&configuration, query).await.unwrap();
    println!("{}", message); 

    // Test#5 search_index
    // This function returns a vector of UUIDs
    println!("____________________________________________________________________________");
    println!("____________________________________________________________________________");
    println!("Test#5 search_index");
    let public_key = search_index_public_key::SearchIndexPublicKey {
        format: search_index_public_key::Format::Ssh,
        content: Some("c3NoLWVkMjU1MTkgQUFBQUMzTnphQzFsWkRJMU5URTVBQUFBSVA3M2tuT0tKYVNyVEtEa2U2OEgvRlJoODRZWU5CU0tBN1hPVWRpWmJjeG8gdGVzdEByZWtvci5kZXYK".to_string()),
        url: None
    };
    let query = SearchIndex {
        email: Some("jpenumak@redhat.com".to_string()),
        public_key: Some(Box::new(public_key)),
        hash: Some("e2535d638859bb63ea9ea5cf467562cba63b007eae1acd0d73a3f259c582561f".to_string())
    };
    let uuid_vec = index_api::search_index(&configuration, query).await;
    println!("{:#?}", uuid_vec);

    // Test#6 get_public_key
    // This function returns the public key as a String
    println!("____________________________________________________________________________");
    println!("____________________________________________________________________________");
    println!("Test#6 get_public_key");
    let pubkey = pubkey_api::get_public_key(&configuration, None).await;
    println!("{:#?}", pubkey);

    // Test#7 get_rekor_version
    // This function returns the rekor version as in a RekorVersion struct.
    println!("____________________________________________________________________________");
    println!("____________________________________________________________________________");
    println!("Test#7 get_rekor_version");
    let rekor_version = server_api::get_rekor_version(&configuration).await;
    println!("{:#?}", rekor_version);

    // Test#8 get_timestamp_cert_chain
    // This function returns the certificate chain as in a String.
    println!("____________________________________________________________________________");
    println!("____________________________________________________________________________");
    println!("Test#8 get_timestamp_cert_chain");
    let cert_chain = timestamp_api::get_timestamp_cert_chain(&configuration).await;
    println!("{}", cert_chain.unwrap());

    // Test#9 : get_timestamp_response needs a parameter request (std::path::PathBuf), not sure what goes into this function	
    println!("____________________________________________________________________________");
    println!("____________________________________________________________________________");
    println!("Test#9 : get_timestamp_response");
    let request_path = PathBuf::from(r"test_request.tsq");
    let response_path = timestamp_api::get_timestamp_response(&configuration, request_path).await.unwrap();
    println!("Rekor's response was saved in: {:#?}", response_path);

    /* // Test#10 get_log_info
    println!("____________________________________________________________________________");
    println!("____________________________________________________________________________");
    println!("Test#10 get_log_info");
    let log_info : LogInfo = tlog_api::get_log_info(&configuration).await.unwrap();
    println!("{:#?}", log_info); */

    // TO DO: figure out why Test #11 fails when I uncomment Test #10
    // Test#11 get_log_proof
    println!("____________________________________________________________________________");
    println!("____________________________________________________________________________");
    println!("Test#11 get_log_proof");
    let log_proof : ConsistencyProof = tlog_api::get_log_proof(&configuration, 10, None, None).await.unwrap();
    println!("{:#?}", log_proof);
}