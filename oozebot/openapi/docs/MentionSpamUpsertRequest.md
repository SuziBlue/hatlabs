# MentionSpamUpsertRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**event_type** | **i32** |  | 
**actions** | Option<[**Vec<models::DefaultKeywordListUpsertRequestActionsInner>**](DefaultKeywordListUpsertRequest_actions_inner.md)> |  | [optional]
**enabled** | Option<**bool**> |  | [optional]
**exempt_roles** | Option<**Vec<String>**> |  | [optional]
**exempt_channels** | Option<**Vec<String>**> |  | [optional]
**trigger_type** | **i32** |  | 
**trigger_metadata** | Option<[**models::MentionSpamTriggerMetadata**](MentionSpamTriggerMetadata.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


