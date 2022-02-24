# \IndexApi

All URIs are relative to *http://rekor.sigstore.dev*

Method | HTTP request | Description
------------- | ------------- | -------------
[**search_index**](IndexApi.md#search_index) | **POST** /api/v1/index/retrieve | Searches index by entry metadata



## search_index

> Vec<String> search_index(query)
Searches index by entry metadata

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | [**SearchIndex**](SearchIndex.md) |  | [required] |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/yaml
- **Accept**: application/json;q=1, application/yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

