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
use rekor::apis::{configuration::Configuration, entries_api};
use rekor::models::{
    hashedrekord::{AlgorithmKind, Data, Hash, PublicKey, Signature, Spec},
    ProposedEntry, SearchLogQuery,
};
use std::str::FromStr;
use url::Url;

#[tokio::main]
async fn main() {
    /*
    Searches transparency log for one or more log entries.
    Returns zero or more entries from the transparency log, according to how many were included in request query.

    Example command :
    cargo run --example search_log_query -- \
     --hash c7ead87fa5c82d2b17feece1c2ee1bda8e94788f4b208de5057b3617a42b7413\
     --url https://raw.githubusercontent.com/jyotsna-penumaka/rekor-rs/rekor-functionality/test_data/data\
     --public_key LS0tLS1CRUdJTiBQVUJMSUMgS0VZLS0tLS0KTUZrd0V3WUhLb1pJemowQ0FRWUlLb1pJemowREFRY0RRZ0FFeEhUTWRSQk80ZThCcGZ3cG5KMlozT2JMRlVrVQpaUVp6WGxtKzdyd1lZKzhSMUZpRWhmS0JZclZraGpHL2lCUjZac2s3Z01iYWZPOG9FM01lUEVvWU93PT0KLS0tLS1FTkQgUFVCTElDIEtFWS0tLS0tCg==\
     --signature MEUCIHWACbBnw+YkJCy2tVQd5i7VH6HgkdVBdP7HRV1IEsDuAiEA19iJNvmkE6We7iZGjHsTkjXV8QhK9iXu0ArUxvJF1N8=\
     --key_format x509\
     --api_version 0.0.1\
     --entry_uuids 1377da9d9dbad451a5a8acdd28add750815d34e8205f1b8a35a67b8a27dae9bf\
     --log_indexes 2922253
     */

    // The following default values will be used if the user does not input values using cli flags
    const HASH: &str = "c7ead87fa5c82d2b17feece1c2ee1bda8e94788f4b208de5057b3617a42b7413";
    const URL: &str = "https://raw.githubusercontent.com/jyotsna-penumaka/rekor-rs/rekor-functionality/test_data/data";
    const PUBLIC_KEY: &str = "LS0tLS1CRUdJTiBQVUJMSUMgS0VZLS0tLS0KTUZrd0V3WUhLb1pJemowQ0FRWUlLb1pJemowREFRY0RRZ0FFeEhUTWRSQk80ZThCcGZ3cG5KMlozT2JMRlVrVQpaUVp6WGxtKzdyd1lZKzhSMUZpRWhmS0JZclZraGpHL2lCUjZac2s3Z01iYWZPOG9FM01lUEVvWU93PT0KLS0tLS1FTkQgUFVCTElDIEtFWS0tLS0tCg==";
    const SIGNATURE: &str = "MEUCIHWACbBnw+YkJCy2tVQd5i7VH6HgkdVBdP7HRV1IEsDuAiEA19iJNvmkE6We7iZGjHsTkjXV8QhK9iXu0ArUxvJF1N8=";
    const KEY_FORMAT: &str = "x509";
    const API_VERSION: &str = "0.0.1";
    const ENTRY_UUIDS: &str = "1377da9d9dbad451a5a8acdd28add750815d34e8205f1b8a35a67b8a27dae9bf";
    const LOG_INDEXES: &str = "2922253";

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
        flags.value_of("hash").unwrap_or(HASH).to_string(),
    );
    let data = Data::new(
        hash,
        Url::parse(flags.value_of("url").unwrap_or(URL)).unwrap(),
    );
    let public_key = PublicKey::new(
        flags
            .value_of("public_key")
            .unwrap_or(PUBLIC_KEY)
            .to_string(),
    );
    let signature = Signature::new(
        flags
            .value_of("key_format")
            .unwrap_or(KEY_FORMAT)
            .to_string(),
        flags.value_of("signature").unwrap_or(SIGNATURE).to_string(),
        public_key,
    );
    let spec = Spec::new(signature, data);
    let proposed_entry = ProposedEntry::Hashedrekord {
        api_version: flags
            .value_of("api_version")
            .unwrap_or(API_VERSION)
            .to_string(),
        spec: spec,
    };

    let query = SearchLogQuery {
        entry_uuids: Some(vec![flags
            .value_of("entry_uuids")
            .unwrap_or(ENTRY_UUIDS)
            .to_string()]),
        log_indexes: Some(vec![i32::from_str(
            flags.value_of("log_indexes").unwrap_or(LOG_INDEXES),
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
