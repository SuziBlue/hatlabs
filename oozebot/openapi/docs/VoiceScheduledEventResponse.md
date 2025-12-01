# VoiceScheduledEventResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**guild_id** | **String** |  | 
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**channel_id** | Option<**String**> |  | [optional]
**creator_id** | Option<**String**> |  | [optional]
**creator** | Option<[**models::UserResponse**](UserResponse.md)> |  | [optional]
**image** | Option<**String**> |  | [optional]
**scheduled_start_time** | **String** |  | 
**scheduled_end_time** | Option<**String**> |  | [optional]
**status** | Option<**i32**> |  | 
**entity_type** | Option<**i32**> |  | 
**entity_id** | Option<**String**> |  | [optional]
**user_count** | Option<**i32**> |  | [optional]
**privacy_level** | Option<[**serde_json::Value**](.md)> |  | 
**user_rsvp** | Option<[**models::ScheduledEventUserResponse**](ScheduledEventUserResponse.md)> |  | [optional]
**entity_metadata** | Option<[**serde_json::Value**](.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


