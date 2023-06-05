# \MatchApi

All URIs are relative to *https://api.xethlyx.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_match**](MatchApi.md#get_match) | **GET** /eclipsis/match/{matchId} | Get match data



## get_match

> crate::models::Success4 get_match(match_id)
Get match data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**match_id** | **String** | Discord snowflake of match to return | [required] |

### Return type

[**crate::models::Success4**](Success_4.md)

### Authorization

[HeaderAuth](../README.md#HeaderAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

