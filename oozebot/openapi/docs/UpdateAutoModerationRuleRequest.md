# UpdateAutoModerationRuleRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**event_type** | Option<**i32**> |  | [optional]
**actions** | Option<[**Vec<models::DefaultKeywordListUpsertRequestActionsInner>**](DefaultKeywordListUpsertRequest_actions_inner.md)> |  | [optional]
**enabled** | Option<**bool**> |  | [optional]
**exempt_roles** | Option<**Vec<String>**> |  | [optional]
**exempt_channels** | Option<**Vec<String>**> |  | [optional]
**trigger_type** | Option<**i32**> |  | [optional]
**trigger_metadata** | Option<[**models::MentionSpamTriggerMetadata**](MentionSpamTriggerMetadata.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


