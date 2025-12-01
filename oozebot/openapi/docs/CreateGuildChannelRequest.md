# CreateGuildChannelRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**i32**> |  | [optional]
**name** | **String** |  | 
**position** | Option<**i32**> |  | [optional]
**topic** | Option<**String**> |  | [optional]
**bitrate** | Option<**i32**> |  | [optional]
**user_limit** | Option<**i32**> |  | [optional]
**nsfw** | Option<**bool**> |  | [optional]
**rate_limit_per_user** | Option<**i32**> |  | [optional]
**parent_id** | Option<**String**> |  | [optional]
**permission_overwrites** | Option<[**Vec<models::ChannelPermissionOverwriteRequest>**](ChannelPermissionOverwriteRequest.md)> |  | [optional]
**rtc_region** | Option<**String**> |  | [optional]
**video_quality_mode** | Option<**i32**> |  | [optional]
**default_auto_archive_duration** | Option<**i32**> |  | [optional]
**default_reaction_emoji** | Option<[**models::UpdateDefaultReactionEmojiRequest**](UpdateDefaultReactionEmojiRequest.md)> |  | [optional]
**default_thread_rate_limit_per_user** | Option<**i32**> |  | [optional]
**default_sort_order** | Option<**i32**> |  | [optional]
**default_forum_layout** | Option<**i32**> |  | [optional]
**default_tag_setting** | Option<**String**> |  | [optional]
**available_tags** | Option<[**Vec<models::CreateOrUpdateThreadTagRequest>**](CreateOrUpdateThreadTagRequest.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


