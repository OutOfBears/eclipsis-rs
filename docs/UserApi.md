# \UserApi

All URIs are relative to *https://api.xethlyx.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_user_delta**](UserApi.md#get_user_delta) | **GET** /eclipsis/user/delta/{userId} | Get rating delta
[**get_user_matches**](UserApi.md#get_user_matches) | **GET** /eclipsis/user/matches/{userId} | Get matches
[**get_user_overview**](UserApi.md#get_user_overview) | **GET** /eclipsis/user/overview/{userId} | Get overview
[**get_user_playtime**](UserApi.md#get_user_playtime) | **GET** /eclipsis/user/playtime/{userId} | Get playtime
[**get_user_rating**](UserApi.md#get_user_rating) | **GET** /eclipsis/user/rating/{userId} | Get rating
[**get_user_status**](UserApi.md#get_user_status) | **POST** /eclipsis/user/status | Get basic status
[**get_user_teammates**](UserApi.md#get_user_teammates) | **GET** /eclipsis/user/teammates/{userId} | Get teammates



## get_user_delta

> crate::models::Success5 get_user_delta(user_id)
Get rating delta

This will return an array of int32 where each value is the change in rating per match.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | Roblox ID of user to return | [required] |

### Return type

[**crate::models::Success5**](Success_5.md)

### Authorization

[HeaderAuth](../README.md#HeaderAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_matches

> crate::models::Success6 get_user_matches(user_id)
Get matches

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | Roblox user ID of user to return | [required] |

### Return type

[**crate::models::Success6**](Success_6.md)

### Authorization

[HeaderAuth](../README.md#HeaderAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_overview

> crate::models::Success7 get_user_overview(user_id)
Get overview

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | Roblox user ID of user to return | [required] |

### Return type

[**crate::models::Success7**](Success_7.md)

### Authorization

[HeaderAuth](../README.md#HeaderAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_playtime

> crate::models::Success8 get_user_playtime(user_id)
Get playtime

Returns the amount of seconds spent in-game. This does not include unlogged matches such as practice, matches played before the embed format, or matches that have crashed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | Roblox user ID of user to return | [required] |

### Return type

[**crate::models::Success8**](Success_8.md)

### Authorization

[HeaderAuth](../README.md#HeaderAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_rating

> crate::models::Success9 get_user_rating(user_id)
Get rating

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | Roblox ID of user to return | [required] |

### Return type

[**crate::models::Success9**](Success_9.md)

### Authorization

[HeaderAuth](../README.md#HeaderAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_status

> crate::models::Success10 get_user_status(request_body)
Get basic status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | [**Vec<i64>**](i64.md) | Roblox user ID of user to return | [required] |

### Return type

[**crate::models::Success10**](Success_10.md)

### Authorization

[HeaderAuth](../README.md#HeaderAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_teammates

> crate::models::Success11 get_user_teammates(user_id)
Get teammates

Gets the most played-with teammates of the specified player

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i64** | Roblox user ID of user to return | [required] |

### Return type

[**crate::models::Success11**](Success_11.md)

### Authorization

[HeaderAuth](../README.md#HeaderAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

