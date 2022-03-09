use openapi::apis::{configuration::Configuration, entries_api};
use openapi::models::{ProposedEntry, log_entry::LogEntry, SearchLogQuery};
use openapi::models::rekord::{Hash, Data, PublicKey, Signature, Spec};
use url::Url;

#[tokio::main]
async fn main() {
    let configuration = Configuration::default();

    // Test create_log_entry 
    println!("Test#1 create_log_entry!");
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

    //let log_entry: LogEntry = entries_api::create_log_entry(&configuration, proposed_entry).await.unwrap();
    //println!("{:#?}", log_entry);

    // Test get_log_entry_by_index 
    println!("Test#2 get_log_entry_by_index");
    //let message: LogEntry = entries_api::get_log_entry_by_index(&configuration, 1).await.unwrap();
    //println!("{:#?}", message);

    // Test get_log_entry_by_uuid 
     println!("Test#3 get_log_entry_by_uuid");
    // let uuid = "073970a07c978b7a9ff15b69fe15d87dfb58fd5756086e3d1fb671c2d0bd95c0";
    // let message: LogEntry = entries_api::get_log_entry_by_uuid(&configuration, &uuid).await.unwrap();
    // println!("{:#?}", message);  

    // Test search_log_query NoneNone
    println!("Test#3 search_log_query");

    let query = SearchLogQuery {
        entry_uuids: Some(vec!["3bb6b37275642a289650d81cd5ba8aba0adcf9b5a5330d21bf9dd6157f4cbb67".to_string()]),
        log_indexes: Some(vec![1594206]),
        entries: Some(vec![proposed_entry]),
    };
    let message = entries_api::search_log_query(&configuration, query).await.unwrap();
    println!("{:#?}", message);

}
