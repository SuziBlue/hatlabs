# GuildTemplateChannelResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional]
**r#type** | **i32** |  | 
**name** | Option<**String**> |  | [optional]
**position** | Option<**i32**> |  | [optional]
**topic** | Option<**String**> |  | [optional]
**bitrate** | **i32** |  | 
**user_limit** | **i32** |  | 
**nsfw** | **bool** |  | 
**rate_limit_per_user** | **i32** |  | 
**parent_id** | Option<**String**> |  | [optional]
**default_auto_archive_duration** | Option<**i32**> |  | [optional]
**permission_overwrites** | [**Vec<models::ChannelPermissionOverwriteResponse>**](ChannelPermissionOverwriteResponse.md) |  | 
**available_tags** | Option<[**Vec<models::GuildTemplateChannelTags>**](GuildTemplateChannelTags.md)> |  | [optional]
**template** | **String** |  | 
**default_reaction_emoji** | Option<[**models::DefaultReactionEmojiResponse**](DefaultReactionEmojiResponse.md)> |  | [optional]
**default_thread_rate_limit_per_user** | Option<**i32**> |  | [optional]
**default_sort_order** | Option<**i32**> |  | [optional]
**default_forum_layout** | Option<**i32**> |  | [optional]
**default_tag_setting** | Option<**String**> |  | [optional]
**icon_emoji** | Option<[**serde_json::Value**](.md)> |  | [optional]
**theme_color** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


