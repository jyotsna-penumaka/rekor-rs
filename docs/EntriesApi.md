# \EntriesApi

All URIs are relative to *http://rekor.sigstore.dev*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_log_entry**](EntriesApi.md#create_log_entry) | **POST** /api/v1/log/entries | Creates an entry in the transparency log
[**get_log_entry_by_index**](EntriesApi.md#get_log_entry_by_index) | **GET** /api/v1/log/entries | Retrieves an entry and inclusion proof from the transparency log (if it exists) by index
[**get_log_entry_by_uuid**](EntriesApi.md#get_log_entry_by_uuid) | **GET** /api/v1/log/entries/{entryUUID} | Get log entry and information required to generate an inclusion proof for the entry in the transparency log
[**search_log_query**](EntriesApi.md#search_log_query) | **POST** /api/v1/log/entries/retrieve | Searches transparency log for one or more log entries



## create_log_entry

> ::std::collections::HashMap<String, serde_json::Value> create_log_entry(proposed_entry)
Creates an entry in the transparency log

Creates an entry in the transparency log for a detached signature, public key, and content. Items can be included in the request or fetched by the server when URLs are specified. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposed_entry** | [**ProposedEntry**](ProposedEntry.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/yaml
- **Accept**: application/json;q=1, application/yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_log_entry_by_index

> ::std::collections::HashMap<String, serde_json::Value> get_log_entry_by_index(log_index)
Retrieves an entry and inclusion proof from the transparency log (if it exists) by index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**log_index** | **i32** | specifies the index of the entry in the transparency log to be retrieved | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;q=1, application/yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_log_entry_by_uuid

> ::std::collections::HashMap<String, serde_json::Value> get_log_entry_by_uuid(entry_uuid)
Get log entry and information required to generate an inclusion proof for the entry in the transparency log

Returns the entry, root hash, tree size, and a list of hashes that can be used to calculate proof of an entry being included in the transparency log

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entry_uuid** | **String** | the UUID of the entry for which the inclusion proof information should be returned | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json;q=1, application/yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_log_query

> Vec<crate::models::Map> search_log_query(entry)
Searches transparency log for one or more log entries

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entry** | [**SearchLogQuery**](SearchLogQuery.md) |  | [required] |

### Return type

[**Vec<crate::models::Map>**](map.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/yaml
- **Accept**: application/json;q=1, application/yaml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

