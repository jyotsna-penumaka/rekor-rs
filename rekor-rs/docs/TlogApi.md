# \TlogApi

All URIs are relative to *http://rekor.sigstore.dev*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_log_info**](TlogApi.md#get_log_info) | **GET** /api/v1/log | Get information about the current state of the transparency log
[**get_log_proof**](TlogApi.md#get_log_proof) | **GET** /api/v1/log/proof | Get information required to generate a consistency proof for the transparency log



## get_log_info

> crate::models::LogInfo get_log_info()
Get information about the current state of the transparency log

Returns the current root hash and size of the merkle tree used to store the log entries.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LogInfo**](LogInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;q=1, application/yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_log_proof

> crate::models::ConsistencyProof get_log_proof(last_size, first_size)
Get information required to generate a consistency proof for the transparency log

Returns a list of hashes for specified tree sizes that can be used to confirm the consistency of the transparency log

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**last_size** | **i32** | The size of the tree that you wish to prove consistency to | [required] |
**first_size** | Option<**i32**> | The size of the tree that you wish to prove consistency from (1 means the beginning of the log) Defaults to 1 if not specified  |  |[default to 1]

### Return type

[**crate::models::ConsistencyProof**](ConsistencyProof.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;q=1, application/yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

