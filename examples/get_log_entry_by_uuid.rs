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
use openapi::models::log_entry::LogEntry;

#[tokio::main]
async fn main() {
    let configuration = Configuration::default();
    let uuid = "073970a07c978b7a9ff15b69fe15d87dfb58fd5756086e3d1fb671c2d0bd95c0";
    let message: LogEntry = entries_api::get_log_entry_by_uuid(&configuration, &uuid)
        .await
        .unwrap();
    println!("{:#?}", message);
}
