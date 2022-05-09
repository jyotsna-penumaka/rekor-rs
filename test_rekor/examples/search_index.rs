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

use openapi::apis::{configuration::Configuration, index_api};
use openapi::models::{search_index_public_key, SearchIndex};

#[tokio::main]
async fn main() {
    let configuration = Configuration::default();
    let public_key = search_index_public_key::SearchIndexPublicKey {
        format: search_index_public_key::Format::Ssh,
        content: Some("c3NoLWVkMjU1MTkgQUFBQUMzTnphQzFsWkRJMU5URTVBQUFBSVA3M2tuT0tKYVNyVEtEa2U2OEgvRlJoODRZWU5CU0tBN1hPVWRpWmJjeG8gdGVzdEByZWtvci5kZXYK".to_string()),
        url: None
    };
    let query = SearchIndex {
        email: Some("jpenumak@redhat.com".to_string()),
        public_key: Some(Box::new(public_key)),
        hash: Some("e2535d638859bb63ea9ea5cf467562cba63b007eae1acd0d73a3f259c582561f".to_string()),
    };
    let uuid_vec = index_api::search_index(&configuration, query).await;
    println!("{:#?}", uuid_vec);
}
