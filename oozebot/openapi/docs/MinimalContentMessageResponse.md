# MinimalContentMessageResponse

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

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


