# \TimestampApi

All URIs are relative to *http://rekor.sigstore.dev*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_timestamp_cert_chain**](TimestampApi.md#get_timestamp_cert_chain) | **GET** /api/v1/timestamp/certchain | Retrieve the certfiicate chain for timestamping that can be used to validate trusted timestamps
[**get_timestamp_response**](TimestampApi.md#get_timestamp_response) | **POST** /api/v1/timestamp | Generates a new timestamp response and creates a new log entry for the timestamp in the transparency log



## get_timestamp_cert_chain

> String get_timestamp_cert_chain()
Retrieve the certfiicate chain for timestamping that can be used to validate trusted timestamps

Returns the certfiicate chain for timestamping that can be used to validate trusted timestamps

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/pem-certificate-chain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_timestamp_response

> std::path::PathBuf get_timestamp_response(request)
Generates a new timestamp response and creates a new log entry for the timestamp in the transparency log

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request** | **std::path::PathBuf** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/timestamp-query
- **Accept**: application/timestamp-reply

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

