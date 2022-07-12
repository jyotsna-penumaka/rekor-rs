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

use rekor::apis::{configuration::Configuration, entries_api};
use rekor::models::{
    hashedrekord::{AlgorithmKind, Data, Hash, PublicKey, Signature, Spec},
    log_entry::LogEntry,
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
     --hash c7ead87fa5c82d2b17feece1c2ee1bda8e94788f4b208de5057b3617a42b7413\
     --url https://raw.githubusercontent.com/jyotsna-penumaka/rekor-rs/rekor-functionality/test_data/data\
     --public_key LS0tLS1CRUdJTiBQVUJMSUMgS0VZLS0tLS0KTUZrd0V3WUhLb1pJemowQ0FRWUlLb1pJemowREFRY0RRZ0FFeEhUTWRSQk80ZThCcGZ3cG5KMlozT2JMRlVrVQpaUVp6WGxtKzdyd1lZKzhSMUZpRWhmS0JZclZraGpHL2lCUjZac2s3Z01iYWZPOG9FM01lUEVvWU93PT0KLS0tLS1FTkQgUFVCTElDIEtFWS0tLS0tCg==\
     --signature MEUCIHWACbBnw+YkJCy2tVQd5i7VH6HgkdVBdP7HRV1IEsDuAiEA19iJNvmkE6We7iZGjHsTkjXV8QhK9iXu0ArUxvJF1N8=\
     --key_format x509\
     --api_version 0.0.1

    create_log_entry will panick if the example code is run,
    with the following error message:

    thread 'main' panicked at 'called `Result::unwrap()` on an `Err`
    value: ResponseError(ResponseContent { status: 409, content: "{\"code\":409,\"message\":\
    "An equivalent entry already exists in the transparency log with UUID
    1377da9d9dbad451a5a8acdd28add750815d34e8205f1b8a35a67b8a27dae9bf\"}\n",
    entity: Some(Status400(Error { code: Some(409), message: Some("An equivalent entry
    already exists in the transparency log with
    UUID 1377da9d9dbad451a5a8acdd28add750815d34e8205f1b8a35a67b8a27dae9bf") })) })',
    examples/create_log_entry.rs:104:10
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    This is because an equivalent entry with the provided meta data already exists in the transparency log.
    When you use the example code to create a new entry with fresh set of input values,
    you should be able to run the code without any errors.

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

    let configuration = Configuration::default();

    let hash = Hash::new(
        AlgorithmKind::sha256,
        flags
            .value_of("hash")
            .unwrap_or("43b8f7d99e183c9be264717ddde94e29cfdd5d4b0e8c3906d815f44be54b76f6")
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
        "LS0tLS1CRUdJTiBQVUJMSUMgS0VZLS0tLS0KTUZrd0V3WUhLb1pJemowQ0FRWUlLb1pJemowREFRY0RRZ0FFdFN1c1doZjZNSzJ0eGpWRHZZRmc1dUovVnhXawpNalh6T2JpV3U2OGh3VG5VdUk0bFppWVJkSjQxd09lMHpwRHl0MFNhVitIOXBkNUVZd1ZTbUxEcjh3PT0KLS0tLS1FTkQgUFVCTElDIEtFWS0tLS0tCg==").to_string(),
    );
    let signature = Signature::new(
        flags.value_of("key_format").unwrap_or("ssh").to_string(),
        flags.value_of("signature").unwrap_or(
        "MEUCIQC5VHZV3DQ4xydZxnmCvOMpi40inNyTfei4p2APp9sN0gIgeNTOQjwp2OCdCaek7l86Y9yL+yxRtqLi3Zbw7/nrPPw=").to_string(),
        public_key,
    );
    let spec = Spec::new(signature, data);
    let proposed_entry = ProposedEntry::Hashedrekord {
        api_version: flags.value_of("api_version").unwrap_or("0.0.1").to_string(),
        spec: spec,
    };

    let log_entry: LogEntry = entries_api::create_log_entry(&configuration, proposed_entry)
        .await
        .unwrap();
    println!("{:#?}", log_entry);
}
