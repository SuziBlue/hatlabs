# RatelimitedResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **i32** | Discord internal error code. See error code reference | 
**message** | **String** | Human-readable error message | 
**retry_after** | **f64** | The number of seconds to wait before retrying your request | 
**global** | **bool** | Whether you are being ratelimited by the global ratelimit or a per-endpoint ratelimit | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


