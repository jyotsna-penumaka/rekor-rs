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
use openapi::apis::{configuration::Configuration, entries_api};
use openapi::models::{
    rekord::{AlgorithmKind, Data, Hash, PublicKey, Signature, Spec},
    ProposedEntry, SearchLogQuery,
};
use std::str::FromStr;
use url::Url;

#[tokio::main]
async fn main() {
    /*
    Searches transparency log for one or more log entries.
    Returns zero or more entries from the transparency log, according to how many were included in request query.
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
    .arg(Arg::new("signature")
             .long("signature")
             .takes_value(true)
             .help("base64 encoded signature of the artifact. Look at https://raw.githubusercontent.com/jyotsna-penumaka/rekor-rs/rekor-functionality/test_data/create_log_entry.md for more details on generating keys."))
    .arg(Arg::new("api_version")
             .long("api_version")
             .takes_value(true)
             .help("Rekor-rs open api version"))
    .arg(Arg::new("entry_uuids")
             .long("entry_uuids")
             .takes_value(true)
             .help("the uuids of the entries to search for"))
    .arg(Arg::new("log_indexes")
            .long("log_indexes")
            .takes_value(true)
            .help("the log_indexes of the entries to search for"));

    let flags = matches.get_matches();

    let hash = Hash::new(
        AlgorithmKind::sha256,
        flags
            .value_of("hash")
            .unwrap_or("27fcf3e4e65e840060efacd20e272917b9571a29eed63e402fd5e1bfb3ba715d")
            .to_string(),
    );
    let data = Data::new(
        hash,
        Url::parse(
            flags.value_of("url").unwrap_or("https://raw.githubusercontent.com/jyotsna-penumaka/rekor-rs/rekor-functionality/test_data/data"),
        )
        .unwrap(),
    );
    let public_key = PublicKey::new(
        flags.value_of("public_key").unwrap_or(
        "c3NoLWVkMjU1MTkgQUFBQUMzTnphQzFsWkRJMU5URTVBQUFBSVA3M2tuT0tKYVNyVEtEa2U2OEgvRlJoODRZWU5CU0tBN1hPVWRpWmJjeG8gdGVzdEByZWtvci5kZXYK").to_string(),
    );
    let signature = Signature::new(
        flags.value_of("public_key").unwrap_or("ssh").to_string(),
        flags.value_of("signature").unwrap_or(
        "LS0tLS1CRUdJTiBTU0ggU0lHTkFUVVJFLS0tLS0KVTFOSVUwbEhBQUFBQVFBQUFETUFBQUFMYzNOb0xXVmtNalUxTVRrQUFBQWcvdmVTYzRvbHBLdE1vT1I3cndmOFZHSHpoaApnMEZJb0R0YzVSMkpsdHpHZ0FBQUFFWm1sc1pRQUFBQUFBQUFBR2MyaGhOVEV5QUFBQVV3QUFBQXR6YzJndFpXUXlOVFV4Ck9RQUFBRURCYVFtVTRXNHFCQzBaM3N6aTZuSEE4cWlBdE5QVzFkU29UTmtlMTBOKzRRTUdad0pRMXR6QTVIYk5BUkxHc3cKN0I0b2RxWWFpRVEwSzMwdEtBZEcwSAotLS0tLUVORCBTU0ggU0lHTkFUVVJFLS0tLS0K").to_string(),
        public_key,
    );
    let spec = Spec::new(signature, data);
    let proposed_entry = ProposedEntry::Rekord {
        api_version: flags.value_of("api_version").unwrap_or("0.0.1").to_string(),
        spec: spec,
    };

    let query = SearchLogQuery {
        entry_uuids: Some(vec![flags
            .value_of("entry_uuids")
            .unwrap_or("3bb6b37275642a289650d81cd5ba8aba0adcf9b5a5330d21bf9dd6157f4cbb67")
            .to_string()]),
        log_indexes: Some(vec![<i32 as FromStr>::from_str(
            flags.value_of("log_indexes").unwrap_or("1594206"),
        )
        .unwrap()]),
        entries: Some(vec![proposed_entry]),
    };

    let configuration = Configuration::default();

    let message = entries_api::search_log_query(&configuration, query)
        .await
        .unwrap();
    println!("{}", message);
}
