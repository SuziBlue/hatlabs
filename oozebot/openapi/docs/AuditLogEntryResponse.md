# AuditLogEntryResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**action_type** | Option<**i32**> |  | 
**user_id** | Option<**String**> |  | [optional]
**target_id** | Option<**String**> |  | [optional]
**changes** | Option<[**Vec<models::AuditLogObjectChangeResponse>**](AuditLogObjectChangeResponse.md)> |  | [optional]
**options** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**reason** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


