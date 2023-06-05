# \ClientApi

All URIs are relative to *https://api.xethlyx.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**client_delete**](ClientApi.md#client_delete) | **DELETE** /client/delete | Delete
[**client_info**](ClientApi.md#client_info) | **GET** /client/info | Info
[**client_login**](ClientApi.md#client_login) | **POST** /client/login | Login
[**client_logout**](ClientApi.md#client_logout) | **POST** /client/logout | Logout



## client_delete

> crate::models::Success client_delete(api_key)
Delete

Someone else leaked their key? Just delete it! 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key** | Option<**String**> |  |  |

### Return type

[**crate::models::Success**](Success.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## client_info

> crate::models::Success1 client_info()
Info

Gets relevant information to this API key.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Success1**](Success_1.md)

### Authorization

[CookieAuth](../README.md#CookieAuth), [HeaderAuth](../README.md#HeaderAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## client_login

> crate::models::Success2 client_login(api_key)
Login

Logs in and saves the key to the browser. If your API key is marked as \"privileged\", refresh the page to automatically load hidden APIs. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key** | Option<**String**> |  |  |

### Return type

[**crate::models::Success2**](Success_2.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## client_logout

> crate::models::Success3 client_logout()
Logout

Logs out from the currently logged in session.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Success3**](Success_3.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

