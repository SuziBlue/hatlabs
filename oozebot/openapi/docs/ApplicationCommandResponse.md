# ApplicationCommandResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**application_id** | **String** |  | 
**version** | **String** |  | 
**default_member_permissions** | Option<**String**> |  | [optional]
**r#type** | **i32** |  | 
**name** | **String** |  | 
**name_localized** | Option<**String**> |  | [optional]
**name_localizations** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**description** | **String** |  | 
**description_localized** | Option<**String**> |  | [optional]
**description_localizations** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**guild_id** | Option<**String**> |  | [optional]
**dm_permission** | Option<**bool**> |  | [optional]
**contexts** | Option<**Vec<i32>**> |  | [optional]
**integration_types** | Option<**Vec<i32>**> |  | [optional]
**options** | Option<[**Vec<models::ApplicationCommandResponseOptionsInner>**](ApplicationCommandResponse_options_inner.md)> |  | [optional]
**nsfw** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


