use openapi::apis::{configuration::Configuration, entries_api, index_api, pubkey_api, server_api, timestamp_api, tlog_api};
use openapi::models::{ProposedEntry, log_entry::LogEntry, SearchLogQuery, SearchIndex, search_index_public_key, LogInfo, ConsistencyProof};
use openapi::models::rekord::{Hash, Data, PublicKey, Signature, Spec};
use url::Url;

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

/*     // Test#1 create_log_entry 
    println!("Test#1 create_log_entry!");
    let log_entry: LogEntry = entries_api::create_log_entry(&configuration, proposed_entry).await.unwrap();
    println!("{:#?}", log_entry); */

/*     // Test#2 get_log_entry_by_index 
    println!("Test#2 get_log_entry_by_index");
    let message: LogEntry = entries_api::get_log_entry_by_index(&configuration, 1).await.unwrap();
    println!("{:#?}", message); */

/*     // Test#3 get_log_entry_by_uuid 
    println!("Test#3 get_log_entry_by_uuid");
    let uuid = "073970a07c978b7a9ff15b69fe15d87dfb58fd5756086e3d1fb671c2d0bd95c0";
    let message: LogEntry = entries_api::get_log_entry_by_uuid(&configuration, &uuid).await.unwrap();
    println!("{:#?}", message);   */

/*     // Test#4 search_log_query
// Note: This needs some work. I was not able to parse the returned response into correct structs!
    println!("Test#4 search_log_query");
    let query = SearchLogQuery {
        entry_uuids: Some(vec!["3bb6b37275642a289650d81cd5ba8aba0adcf9b5a5330d21bf9dd6157f4cbb67".to_string()]),
        log_indexes: Some(vec![1594206]),
        entries: Some(vec![proposed_entry]),
    };
    let message = entries_api::search_log_query(&configuration, query).await;
    println!("{:#?}", message);  */

/*     // Test#5 search_index
    // This function returns a vector of UUIDs
    println!("Test#5 search_index");
    let public_key = search_index_public_key::SearchIndexPublicKey {
        format: search_index_public_key::Format::Ssh,
        content: Some("c3NoLWVkMjU1MTkgQUFBQUMzTnphQzFsWkRJMU5URTVBQUFBSVA3M2tuT0tKYVNyVEtEa2U2OEgvRlJoODRZWU5CU0tBN1hPVWRpWmJjeG8gdGVzdEByZWtvci5kZXYK".to_string()),
        url: None
    };
    let query = SearchIndex {
        ema{
	"signatures": [
		{
			"keyid": "2f64fb5eac0cf94dd39bb45308b98920055e9a0d8e012a7220787834c60aef97",
			"sig": "3044022079252576532ed5ed4a19e4135997d89172101ed745a4489be6b20d04d483bbcc0220515119aab690033dc1e1650f08995dc839dcd161cab3898db0749063ca32dc86"
		},
		{
			"keyid": "bdde902f5ec668179ff5ca0dabf7657109287d690bf97e230c21d65f99155c62",
			"sig": "3045022100c5216dd17d381c951b5174f8ee2157b315d1f26247e7f9e49c42cf975dfcf49b022048fb1751a86fddedc21129e94a3e7e0efeeb93f1238fad6636bbf0c0d39543e8"
		},
		{
			"keyid": "eaf22372f417dd618a46f6c627dbc276e9fd30a004fc94f9be946e73f8bd090b",
			"sig": "3045022042c6b4003deee27db7db6f5aebb29ac89625fd7389dfff434fa93c65cf8aed5f022100fe6cbd036b5fce1169d7392ecfaf76e01f05fdc6c81cf9bae8c9227fc09c65d9"
		},
		{
			"keyid": "f40f32044071a9365505da3d1e3be6561f6f22d0e60cf51df783999f6c3429cb",
			"sig": "3045022100bc431b7315c2aa657418835005692021de7496bbc7c1a2fedf2aafe8d861ca5402200cbca80a4555d8236265e1b746743532894b46257c450ff8706d0e50e659978c"
		},
		{
			"keyid": "f505595165a177a41750a8e864ed1719b1edfccd5a426fd2c0ffda33ce7ff209",
			"sig": "3046022100c09aea2f05e94a656bd70379340c7b5f09b24bbd20adf4855be3783d7ce39482022100da93a8a1577599979d38bfc44016bde5838d9548797fc9e960780276855bbcf9"
		}
	],
	"signed": {
		"_type": "timestamp",
		"expires": "2021-12-18T13:28:12.99008-06:00",
		"meta": {
			"snapshot.json": {
				"hashes": {
					"sha512": "9103503c18f7da2098dce04892948ad240c1b9965048c4ab4da0c32248f3491652d91d76fe9022be2cf15a99e68b3a3ddd1034e5293c8aac86d0446c4354716d"
				},
				"length": 1849,
				"version": 1
			}
		},
		"spec_version": "1.0",
		"version": 1
	}
}il: Some("jpenumak@redhat.com".to_string()),
        public_key: Some(Box::new(public_key)),
        hash: Some("e2535d638859bb63ea9ea5cf467562cba63b007eae1acd0d73a3f259c582561f".to_string()),
    };
    let uuid_vec = index_api::search_index(&configuration, query).await; */

/*     // Test#6 get_public_key
    // This function returns the public key as a String
    println!("Test#6 get_public_key");
    let pubkey = pubkey_api::get_public_key(&configuration).await;
    println!("{:#?}", pubkey); */

/*     // Test#7 get_public_key
    // This function returns the public key as in a RekorVersion struct.
    println!("Test#7 get_rekor_version");
    let rekor_version = server_api::get_rekor_version(&configuration).await;
    println!("{:#?}", rekor_version); */

/*      // Test#8 get_timestamp_cert_chain
    // This function returns the certificate chain as in a String.
    println!("Test#8 get_timestamp_cert_chain");
    let cert_chain = timestamp_api::get_timestamp_cert_chain(&configuration).await;
    println!("{}", cert_chain.unwrap()); */

    // Test#9 : 

/*     // Test#10 get_log_info
    println!("Test#10 get_log_info");
    let log_info : LogInfo = tlog_api::get_log_info(&configuration).await.unwrap();
    println!("{:#?}", log_info); */

    // Test#11 get_log_proof
    println!("Test#11 get_log_proof");
    let log_proof : ConsistencyProof = tlog_api::get_log_proof(&configuration, 1236200, None).await.unwrap();
    println!("{:#?}", log_proof);

}
