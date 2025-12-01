# PrivateApplicationResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**name** | **String** |  | 
**icon** | Option<**String**> |  | [optional]
**description** | **String** |  | 
**r#type** | Option<**i32**> |  | [optional]
**cover_image** | Option<**String**> |  | [optional]
**primary_sku_id** | Option<**String**> |  | [optional]
**bot** | Option<[**models::UserResponse**](UserResponse.md)> |  | [optional]
**slug** | Option<**String**> |  | [optional]
**guild_id** | Option<**String**> |  | [optional]
**rpc_origins** | Option<**Vec<String>**> |  | [optional]
**bot_public** | Option<**bool**> |  | [optional]
**bot_require_code_grant** | Option<**bool**> |  | [optional]
**terms_of_service_url** | Option<**String**> |  | [optional]
**privacy_policy_url** | Option<**String**> |  | [optional]
**custom_install_url** | Option<**String**> |  | [optional]
**install_params** | Option<[**models::ApplicationOAuth2InstallParamsResponse**](ApplicationOAuth2InstallParamsResponse.md)> |  | [optional]
**integration_types_config** | Option<[**std::collections::HashMap<String, models::ApplicationIntegrationTypeConfigurationResponse>**](ApplicationIntegrationTypeConfigurationResponse.md)> |  | [optional]
**verify_key** | **String** |  | 
**flags** | **i32** |  | 
**max_participants** | Option<**i32**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**redirect_uris** | **Vec<String>** |  | 
**interactions_endpoint_url** | Option<**String**> |  | [optional]
**role_connections_verification_url** | Option<**String**> |  | [optional]
**owner** | [**models::UserResponse**](UserResponse.md) |  | 
**approximate_guild_count** | Option<**i32**> |  | [optional]
**approximate_user_install_count** | **i32** |  | 
**approximate_user_authorization_count** | **i32** |  | 
**explicit_content_filter** | **i32** |  | 
**team** | Option<[**models::TeamResponse**](TeamResponse.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


