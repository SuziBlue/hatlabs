#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize)]
pub struct LiveChatMessageListResponse {
    pub kind: String,
    pub etag: Option<String>,
    pub nextPageToken: Option<String>,
    pub pollingIntervalMillis: Option<u64>,
    pub offlineAt: Option<String>,
    pub pageInfo: Option<PageInfo>,
    pub items: Vec<LiveChatMessage>,
    pub activePollItem: Option<LiveChatMessage>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LiveChatMessage {
    pub kind: String,
    pub etag: String,
    pub id: String,
    pub snippet: Snippet,
    pub authorDetails: AuthorDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Snippet {
    #[serde(rename = "type")]
    pub type_field: String,
    pub liveChatId: String,
    pub authorChannelId: String,
    pub publishedAt: String, // Could also use chrono::DateTime if parsing dates
    pub hasDisplayContent: bool,
    pub displayMessage: String,

    pub fanFundingEventDetails: Option<FanFundingEventDetails>,
    pub textMessageDetails: Option<TextMessageDetails>,
    pub messageDeletedDetails: Option<MessageDeletedDetails>,
    pub userBannedDetails: Option<UserBannedDetails>,
    pub memberMilestoneChatDetails: Option<MemberMilestoneChatDetails>,
    pub newSponsorDetails: Option<NewSponsorDetails>,
    pub superChatDetails: Option<SuperChatDetails>,
    pub superStickerDetails: Option<SuperStickerDetails>,
    pub pollDetails: Option<PollDetails>,
    pub membershipGiftingDetails: Option<MembershipGiftingDetails>,
    pub giftMembershipReceivedDetails: Option<GiftMembershipReceivedDetails>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FanFundingEventDetails {
    pub amountMicros: u64,
    pub currency: String,
    pub amountDisplayString: String,
    pub userComment: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextMessageDetails {
    pub messageText: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageDeletedDetails {
    pub deletedMessageId: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserBannedDetails {
    pub bannedUserDetails: BannedUserDetails,
    pub banType: String,
    pub banDurationSeconds: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BannedUserDetails {
    pub channelId: String,
    pub channelUrl: String,
    pub displayName: String,
    pub profileImageUrl: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberMilestoneChatDetails {
    pub userComment: String,
    pub memberMonth: u32,
    pub memberLevelName: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewSponsorDetails {
    pub memberLevelName: String,
    pub isUpgrade: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuperChatDetails {
    pub amountMicros: u64,
    pub currency: String,
    pub amountDisplayString: String,
    pub userComment: String,
    pub tier: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuperStickerDetails {
    pub superStickerMetadata: SuperStickerMetadata,
    pub amountMicros: u64,
    pub currency: String,
    pub amountDisplayString: String,
    pub tier: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuperStickerMetadata {
    pub stickerId: String,
    pub altText: String,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PollDetails {
    pub metadata: PollMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PollMetadata {
    pub options: Vec<PollOption>,
    pub questionText: String,
    pub status: String, // Could be an enum if values are known
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PollOption {
    pub optionText: String,
    pub tally: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MembershipGiftingDetails {
    pub giftMembershipsCount: i32,
    pub giftMembershipsLevelName: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GiftMembershipReceivedDetails {
    pub memberLevelName: String,
    pub gifterChannelId: String,
    pub associatedMembershipGiftingMessageId: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorDetails {
    pub channelId: String,
    pub channelUrl: String,
    pub displayName: String,
    pub profileImageUrl: String,
    pub isVerified: bool,
    pub isChatOwner: bool,
    pub isChatSponsor: bool,
    pub isChatModerator: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageInfo {
    pub totalResults: u32,
    pub resultsPerPage: u32,
}
