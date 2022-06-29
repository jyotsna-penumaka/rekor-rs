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

use clap::{Arg, Command};
use openapi::apis::{configuration::Configuration, index_api};
use openapi::models::{search_index_public_key, search_index_public_key::Format, SearchIndex};

#[tokio::main]
async fn main() {
    /*
    Example command:
    cargo run --example search_index -- \
    --hash e2535d638859bb63ea9ea5cf467562cba63b007eae1acd0d73a3f259c582561f \
    --public_key c3NoLWVkMjU1MTkgQUFBQUMzTnphQzFsWkRJMU5URTVBQUFBSVA3M2tuT0tKYVNyVEtEa2U2OEgvRlJoODRZWU5CU0tBN1hPVWRpWmJjeG8gdGVzdEByZWtvci5kZXYK \
    --key_format ssh \
    --email jpenumak@redhat.com
    */

    let matches = Command::new("cmd")
    .arg(Arg::new("hash")
             .long("hash")
             .takes_value(true)
             .help("hash of the artifact"))
    .arg(Arg::new("url")
             .long("url")
             .takes_value(true)
             .help("url containing the contents of the artifact (raw github url)"))
    .arg(Arg::new("public_key")
             .long("public_key")
             .takes_value(true)
             .help("base64 encoded public_key. Look at https://raw.githubusercontent.com/jyotsna-penumaka/rekor-rs/rekor-functionality/test_data/create_log_entry.md for more details on generating keys."))
    .arg(Arg::new("key_format")
             .long("key_format")
             .takes_value(true)
             .help("Accepted formats are : pgp / x509 / minsign / ssh / tuf"))  
     .arg(Arg::new("email")
             .long("email")
             .takes_value(true)
             .help("Author's email"));

    let flags = matches.get_matches();

    let key_format = match flags.value_of("key_format").unwrap() {
        "pgp" => Format::Pgp,
        "x509" => Format::X509,
        "minisign" => Format::Minisign,
        "ssh" => Format::Ssh,
        _ => Format::Tuf,
    };

    let public_key = search_index_public_key::SearchIndexPublicKey {
        format: key_format,
        content: Some(flags.value_of("public_key").unwrap().to_string()),
        url: None,
    };

    let query = SearchIndex {
        email: Some(flags.value_of("email").unwrap().to_string()),
        public_key: Some(Box::new(public_key)),
        hash: Some(flags.value_of("hash").unwrap().to_string()),
    };
    let configuration = Configuration::default();

    let uuid_vec = index_api::search_index(&configuration, query).await;
    println!("{:#?}", uuid_vec);
}
