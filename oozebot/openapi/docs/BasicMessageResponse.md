# BasicMessageResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **i32** |  | 
**content** | **String** |  | 
**mentions** | [**Vec<models::UserResponse>**](UserResponse.md) |  | 
**mention_roles** | **Vec<String>** |  | 
**attachments** | [**Vec<models::MessageAttachmentResponse>**](MessageAttachmentResponse.md) |  | 
**embeds** | [**Vec<models::MessageEmbedResponse>**](MessageEmbedResponse.md) |  | 
**timestamp** | **String** |  | 
**edited_timestamp** | Option<**String**> |  | [optional]
**flags** | **i32** |  | 
**components** | [**Vec<models::BasicMessageResponseComponentsInner>**](BasicMessageResponse_components_inner.md) |  | 
**stickers** | Option<[**Vec<models::GetSticker200Response>**](get_sticker_200_response.md)> |  | [optional]
**sticker_items** | Option<[**Vec<models::MessageStickerItemResponse>**](MessageStickerItemResponse.md)> |  | [optional]
**id** | **String** |  | 
**channel_id** | **String** |  | 
**author** | [**models::UserResponse**](UserResponse.md) |  | 
**pinned** | **bool** |  | 
**mention_everyone** | **bool** |  | 
**tts** | **bool** |  | 
**call** | Option<[**models::MessageCallResponse**](MessageCallResponse.md)> |  | [optional]
**activity** | Option<[**serde_json::Value**](.md)> |  | [optional]
**application** | Option<[**models::BasicApplicationResponse**](BasicApplicationResponse.md)> |  | [optional]
**application_id** | Option<**String**> |  | [optional]
**interaction** | Option<[**models::MessageInteractionResponse**](MessageInteractionResponse.md)> |  | [optional]
**nonce** | Option<[**models::BasicMessageResponseNonce**](BasicMessageResponse_nonce.md)> |  | [optional]
**webhook_id** | Option<**String**> |  | [optional]
**message_reference** | Option<[**models::MessageReferenceResponse**](MessageReferenceResponse.md)> |  | [optional]
**thread** | Option<[**models::ThreadResponse**](ThreadResponse.md)> |  | [optional]
**mention_channels** | Option<[**Vec<models::MessageMentionChannelResponse>**](MessageMentionChannelResponse.md)> |  | [optional]
**role_subscription_data** | Option<[**models::MessageRoleSubscriptionDataResponse**](MessageRoleSubscriptionDataResponse.md)> |  | [optional]
**purchase_notification** | Option<[**models::PurchaseNotificationResponse**](PurchaseNotificationResponse.md)> |  | [optional]
**position** | Option<**i32**> |  | [optional]
**resolved** | Option<[**models::ResolvedObjectsResponse**](ResolvedObjectsResponse.md)> |  | [optional]
**poll** | Option<[**models::PollResponse**](PollResponse.md)> |  | [optional]
**shared_client_theme** | Option<[**models::CustomClientThemeResponse**](CustomClientThemeResponse.md)> |  | [optional]
**interaction_metadata** | Option<[**models::BasicMessageResponseInteractionMetadata**](BasicMessageResponse_interaction_metadata.md)> |  | [optional]
**message_snapshots** | Option<[**Vec<models::MessageSnapshotResponse>**](MessageSnapshotResponse.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


