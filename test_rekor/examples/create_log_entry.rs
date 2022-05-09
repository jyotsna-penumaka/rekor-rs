use openapi::apis::{configuration::Configuration, entries_api};
use openapi::models::{
    log_entry::LogEntry,
    rekord::{Data, Hash, PublicKey, Signature, Spec, AlgorithmKind},
    ProposedEntry,
};
use url::Url;

#[tokio::main]
async fn main() {
    let configuration = Configuration::default();
    let hash = Hash::new(
        AlgorithmKind::sha256,
        "e2535d638859bb63ea9ea5cf467562cba63b007eae1acd0d73a3f259c582561f".to_string(),
    );
    let data = Data::new(
        hash,
        Url::parse(
            "https://raw.githubusercontent.com/jyotsna-penumaka/integrate-rekor/main/README.md",
        )
        .unwrap(),
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
    let proposed_entry = ProposedEntry::Rekord {
        api_version: "0.0.1".to_string(),
        spec: spec,
    };

    let log_entry: LogEntry = entries_api::create_log_entry(&configuration, proposed_entry)
        .await
        .unwrap();
    println!("{:#?}", log_entry);
}
