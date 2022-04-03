# InclusionProof

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**log_index** | **i32** | The index of the entry in the transparency log | 
**root_hash** | **String** | The hash value stored at the root of the merkle tree at the time the proof was generated | 
**tree_size** | **i32** | The size of the merkle tree at the time the inclusion proof was generated | 
**hashes** | **Vec<String>** | A list of hashes required to compute the inclusion proof, sorted in order from leaf to root | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


