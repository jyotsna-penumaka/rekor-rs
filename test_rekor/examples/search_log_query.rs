//
// Copyright 2021 The Sigstore Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use openapi::apis::{configuration::Configuration, entries_api};
use openapi::models::{
    rekord::{AlgorithmKind, Data, Hash, PublicKey, Signature, Spec},
    ProposedEntry, SearchLogQuery,
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

    let query = SearchLogQuery {
        entry_uuids: Some(vec![
            "3bb6b37275642a289650d81cd5ba8aba0adcf9b5a5330d21bf9dd6157f4cbb67".to_string(),
        ]),
        log_indexes: Some(vec![1594206]),
        entries: Some(vec![proposed_entry]),
    };

    let message = entries_api::search_log_query(&configuration, query)
        .await
        .unwrap();
    println!("{}", message);
}
