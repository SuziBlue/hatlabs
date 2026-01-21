use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DispatchEvent {
    Ready,
    Resumed,

    ApplicationCommandPermissionsUpdate,

    AutoModerationRuleCreate,
    AutoModerationRuleUpdate,
    AutoModerationRuleDelete,
    AutoModerationActionExecution,

    ChannelCreate,
    ChannelUpdate,
    ChannelDelete,
    ChannelPinsUpdate,

    ThreadCreate,
    ThreadUpdate,
    ThreadDelete,
    ThreadListSync,
    ThreadMemberUpdate,
    ThreadMembersUpdate,

    EntitlementCreate,
    EntitlementUpdate,
    EntitlementDelete,

    GuildCreate,
    GuildUpdate,
    GuildDelete,
    GuildAuditLogEntryCreate,
    GuildBanAdd,
    GuildBanRemove,
    GuildEmojisUpdate,
    GuildStickersUpdate,
    GuildIntegrationsUpdate,
    GuildMemberAdd,
    GuildMemberRemove,
    GuildMemberUpdate,
    GuildMembersChunk,
    GuildRoleCreate,
    GuildRoleUpdate,
    GuildRoleDelete,
    GuildScheduledEventCreate,
    GuildScheduledEventUpdate,
    GuildScheduledEventDelete,
    GuildScheduledEventUserAdd,
    GuildScheduledEventUserRemove,
    GuildSoundboardSoundCreate,
    GuildSoundboardSoundUpdate,
    GuildSoundboardSoundDelete,
    GuildSoundboardSoundsUpdate,

    SoundboardSounds,

    IntegrationCreate,
    IntegrationUpdate,
    IntegrationDelete,

    InteractionCreate,

    InviteCreate,
    InviteDelete,

    MessageCreate,
    MessageUpdate,
    MessageDelete,
    MessageDeleteBulk,
    MessageReactionAdd,
    MessageReactionRemove,
    MessageReactionRemoveAll,
    MessageReactionRemoveEmoji,
    MessagePollVoteAdd,
    MessagePollVoteRemove,

    PresenceUpdate,

    StageInstanceCreate,
    StageInstanceUpdate,
    StageInstanceDelete,

    SubscriptionCreate,
    SubscriptionUpdate,
    SubscriptionDelete,

    TypingStart,

    UserUpdate,

    VoiceChannelEffectSend,
    VoiceStateUpdate,
    VoiceServerUpdate,

    WebhooksUpdate,

    #[serde(other)]
    Unknown,
}
