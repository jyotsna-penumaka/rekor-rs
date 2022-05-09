# rekor-rs

Continuous integration | Docs | License
 ----------------------|------|---------
 [![Continuous integration](https://github.com/sigstore/sigstore-rs/actions/workflows/tests.yml/badge.svg)](https://github.com/sigstore/sigstore-rs/actions/workflows/tests.yml) | [![Docs](https://img.shields.io/badge/docs-%20-blue)](https://sigstore.github.io/sigstore-rs/sigstore) |  [![License: Apache 2.0](https://img.shields.io/badge/License-Apache2.0-brightgreen.svg)](https://opensource.org/licenses/Apache-2.0)

In order to test the code :-
```
cd test_rekor
```
In order to view the available examples:
```
$ cargo run --example

error: "--example" takes one argument.
Available examples:
    create_log_entry
    get_log_entry_by_index
    get_log_entry_by_uuid
    get_log_info
    get_log_proof
    get_public_key
    get_rekor_version
    get_timestamp_cert_chain
    get_timestamp_response
    search_index
    search_log_query

```

Run individual examples and view the output with:
```
cargo run --example get_log_entry_by_index
```
