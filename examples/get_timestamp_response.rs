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
use openapi::apis::{configuration::Configuration, timestamp_api};
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    /*

    Breaking change: timestamping authority was removed #813
    https://github.com/sigstore/rekor/pull/813
    Therefore, this code will result in a kernel panick

    Example command :
    cargo run --example get_public_key
    */

    let matches = Command::new("cmd").arg(
        Arg::new("request_path")
            .long("request_path")
            .takes_value(true)
            .help("The path the file containing the request"),
    );

    let flags = matches.get_matches();
    let configuration = Configuration::default();
    let request_path = PathBuf::from(r"test_data/test_request.tsq");

    let response_path = timestamp_api::get_timestamp_response(
        &configuration,
        PathBuf::from(
            flags
                .value_of("request_path")
                .unwrap_or(r"test_data/test_request.tsq"),
        ),
    )
    .await
    .unwrap();

    println!("Rekor's response was saved in: {:#?}", response_path);
}
