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
