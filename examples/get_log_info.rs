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

use rekor::apis::{configuration::Configuration, tlog_api};
use rekor::models::LogInfo;

#[tokio::main]
async fn main() {
    /*
    Get information about the current state of the transparency log.
    Returns the current root hash and size of the merkle tree used to store the log entries.
    */
    let configuration = Configuration::default();
    let log_info: LogInfo = tlog_api::get_log_info(&configuration).await.unwrap();
    println!("{:#?}", log_info);
}
