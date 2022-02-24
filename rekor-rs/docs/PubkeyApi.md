# \PubkeyApi

All URIs are relative to *http://rekor.sigstore.dev*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_public_key**](PubkeyApi.md#get_public_key) | **GET** /api/v1/log/publicKey | Retrieve the public key that can be used to validate the signed tree head



## get_public_key

> String get_public_key()
Retrieve the public key that can be used to validate the signed tree head

Returns the public key that can be used to validate the signed tree head

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-pem-file

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

