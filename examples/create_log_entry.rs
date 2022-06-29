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
    log_entry::LogEntry,
    rekord::{AlgorithmKind, Data, Hash, PublicKey, Signature, Spec},
    ProposedEntry,
};
use url::Url;

use clap::{Arg, Command};

#[tokio::main]
async fn main() {
    /*
    Creates an entry in the transparency log for a detached signature, public key, and content.
    Items can be included in the request or fetched by the server when URLs are specified.

    Example command :
    cargo run --example create_log_entry -- \
     --hash 27b46cbb8b82ea64b6c8f0ba592b891e288fd24211516a7256d769463404fe7c\
     --url https://raw.githubusercontent.com/jyotsna-penumaka/rekor-rs/rekor-functionality/test_data/data\
     --public_key c3NoLWVkMjU1MTkgQUFBQUMzTnphQzFsWkRJMU5URTVBQUFBSVA3M2tuT0tKYVNyVEtEa2U2OEgvRlJoODRZWU5CU0tBN1hPVWRpWmJjeG8gdGVzdEByZWtvci5kZXYK\
     --signature LS0tLS1CRUdJTiBTU0ggU0lHTkFUVVJFLS0tLS0KVTFOSVUwbEhBQUFBQVFBQUFETUFBQUFMYzNOb0xXVmtNalUxTVRrQUFBQWcvdmVTYzRvbHBLdE1vT1I3cndmOFZHSHpoaApnMEZJb0R0YzVSMkpsdHpHZ0FBQUFFWm1sc1pRQUFBQUFBQUFBR2MyaGhOVEV5QUFBQVV3QUFBQXR6YzJndFpXUXlOVFV4Ck9RQUFBRUJqc2pZdVhYcDliVGVtZ0xLMDk3TUZMNTlvZCtEd1NrU3NHWFQ5UUZsMDFnRVYzK2R5VmRBY3lGUWo0TmErY0cKVThtOVZhQXVhR1JZblJLUUpPNEs0QgotLS0tLUVORCBTU0ggU0lHTkFUVVJFLS0tLS0K\
     --key_format ssh\
     --api_version 0.0.1
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
             .help("Rekor-rs open api version"));

    let flags = matches.get_matches();

    //println!("{}", flags.value_of("api_version").unwrap_or("aaa"));

    let configuration = Configuration::default();

    let hash = Hash::new(
        AlgorithmKind::sha256,
        flags
            .value_of("hash")
            .unwrap_or("27b46cbb8b82ea64b6c8f0ba592b891e288fd24211516a7256d769463404fe7c")
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
        flags.value_of("key_format").unwrap_or("ssh").to_string(),
        flags.value_of("signature").unwrap_or(
        "LS0tLS1CRUdJTiBTU0ggU0lHTkFUVVJFLS0tLS0KVTFOSVUwbEhBQUFBQVFBQUFETUFBQUFMYzNOb0xXVmtNalUxTVRrQUFBQWcvdmVTYzRvbHBLdE1vT1I3cndmOFZHSHpoaApnMEZJb0R0YzVSMkpsdHpHZ0FBQUFFWm1sc1pRQUFBQUFBQUFBR2MyaGhOVEV5QUFBQVV3QUFBQXR6YzJndFpXUXlOVFV4Ck9RQUFBRUJqc2pZdVhYcDliVGVtZ0xLMDk3TUZMNTlvZCtEd1NrU3NHWFQ5UUZsMDFnRVYzK2R5VmRBY3lGUWo0TmErY0cKVThtOVZhQXVhR1JZblJLUUpPNEs0QgotLS0tLUVORCBTU0ggU0lHTkFUVVJFLS0tLS0K").to_string(),
        public_key,
    );
    let spec = Spec::new(signature, data);
    let proposed_entry = ProposedEntry::Rekord {
        api_version: flags.value_of("api_version").unwrap_or("0.0.1").to_string(),
        spec: spec,
    };

    let log_entry: LogEntry = entries_api::create_log_entry(&configuration, proposed_entry)
        .await
        .unwrap();
    println!("{:#?}", log_entry);
}
