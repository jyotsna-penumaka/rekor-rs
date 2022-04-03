/*
 * Rekor
 *
 * Rekor is a cryptographically secure, immutable transparency log for signed software releases.
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConsistencyProof {
    /// The hash value stored at the root of the merkle tree at the time the proof was generated
    #[serde(rename = "rootHash")]
    pub root_hash: String,
    #[serde(rename = "hashes")]
    pub hashes: Vec<String>,
}

impl ConsistencyProof {
    pub fn new(root_hash: String, hashes: Vec<String>) -> ConsistencyProof {
        ConsistencyProof {
            root_hash,
            hashes,
        }
    }
}


