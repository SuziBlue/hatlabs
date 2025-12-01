# \DefaultApi

All URIs are relative to *https://discord.com/api/v10*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_group_dm_user**](DefaultApi.md#add_group_dm_user) | **PUT** /channels/{channel_id}/recipients/{user_id} | 
[**add_guild_member**](DefaultApi.md#add_guild_member) | **PUT** /guilds/{guild_id}/members/{user_id} | 
[**add_guild_member_role**](DefaultApi.md#add_guild_member_role) | **PUT** /guilds/{guild_id}/members/{user_id}/roles/{role_id} | 
[**add_lobby_member**](DefaultApi.md#add_lobby_member) | **PUT** /lobbies/{lobby_id}/members/{user_id} | 
[**add_my_message_reaction**](DefaultApi.md#add_my_message_reaction) | **PUT** /channels/{channel_id}/messages/{message_id}/reactions/{emoji_name}/@me | 
[**add_thread_member**](DefaultApi.md#add_thread_member) | **PUT** /channels/{channel_id}/thread-members/{user_id} | 
[**applications_get_activity_instance**](DefaultApi.md#applications_get_activity_instance) | **GET** /applications/{application_id}/activity-instances/{instance_id} | 
[**ban_user_from_guild**](DefaultApi.md#ban_user_from_guild) | **PUT** /guilds/{guild_id}/bans/{user_id} | 
[**bot_partner_sdk_token**](DefaultApi.md#bot_partner_sdk_token) | **POST** /partner-sdk/token/bot | 
[**bot_partner_sdk_unmerge_provisional_account**](DefaultApi.md#bot_partner_sdk_unmerge_provisional_account) | **POST** /partner-sdk/provisional-accounts/unmerge/bot | 
[**bulk_ban_users_from_guild**](DefaultApi.md#bulk_ban_users_from_guild) | **POST** /guilds/{guild_id}/bulk-ban | 
[**bulk_delete_messages**](DefaultApi.md#bulk_delete_messages) | **POST** /channels/{channel_id}/messages/bulk-delete | 
[**bulk_set_application_commands**](DefaultApi.md#bulk_set_application_commands) | **PUT** /applications/{application_id}/commands | 
[**bulk_set_guild_application_commands**](DefaultApi.md#bulk_set_guild_application_commands) | **PUT** /applications/{application_id}/guilds/{guild_id}/commands | 
[**bulk_update_guild_channels**](DefaultApi.md#bulk_update_guild_channels) | **PATCH** /guilds/{guild_id}/channels | 
[**bulk_update_guild_roles**](DefaultApi.md#bulk_update_guild_roles) | **PATCH** /guilds/{guild_id}/roles | 
[**bulk_update_lobby_members**](DefaultApi.md#bulk_update_lobby_members) | **POST** /lobbies/{lobby_id}/members/bulk | 
[**consume_entitlement**](DefaultApi.md#consume_entitlement) | **POST** /applications/{application_id}/entitlements/{entitlement_id}/consume | 
[**create_application_command**](DefaultApi.md#create_application_command) | **POST** /applications/{application_id}/commands | 
[**create_application_emoji**](DefaultApi.md#create_application_emoji) | **POST** /applications/{application_id}/emojis | 
[**create_auto_moderation_rule**](DefaultApi.md#create_auto_moderation_rule) | **POST** /guilds/{guild_id}/auto-moderation/rules | 
[**create_channel_invite**](DefaultApi.md#create_channel_invite) | **POST** /channels/{channel_id}/invites | 
[**create_dm**](DefaultApi.md#create_dm) | **POST** /users/@me/channels | 
[**create_entitlement**](DefaultApi.md#create_entitlement) | **POST** /applications/{application_id}/entitlements | 
[**create_guild_application_command**](DefaultApi.md#create_guild_application_command) | **POST** /applications/{application_id}/guilds/{guild_id}/commands | 
[**create_guild_channel**](DefaultApi.md#create_guild_channel) | **POST** /guilds/{guild_id}/channels | 
[**create_guild_emoji**](DefaultApi.md#create_guild_emoji) | **POST** /guilds/{guild_id}/emojis | 
[**create_guild_role**](DefaultApi.md#create_guild_role) | **POST** /guilds/{guild_id}/roles | 
[**create_guild_scheduled_event**](DefaultApi.md#create_guild_scheduled_event) | **POST** /guilds/{guild_id}/scheduled-events | 
[**create_guild_soundboard_sound**](DefaultApi.md#create_guild_soundboard_sound) | **POST** /guilds/{guild_id}/soundboard-sounds | 
[**create_guild_sticker**](DefaultApi.md#create_guild_sticker) | **POST** /guilds/{guild_id}/stickers | 
[**create_guild_template**](DefaultApi.md#create_guild_template) | **POST** /guilds/{guild_id}/templates | 
[**create_interaction_response**](DefaultApi.md#create_interaction_response) | **POST** /interactions/{interaction_id}/{interaction_token}/callback | 
[**create_linked_lobby_guild_invite_for_self**](DefaultApi.md#create_linked_lobby_guild_invite_for_self) | **POST** /lobbies/{lobby_id}/members/@me/invites | 
[**create_linked_lobby_guild_invite_for_user**](DefaultApi.md#create_linked_lobby_guild_invite_for_user) | **POST** /lobbies/{lobby_id}/members/{user_id}/invites | 
[**create_lobby**](DefaultApi.md#create_lobby) | **POST** /lobbies | 
[**create_lobby_message**](DefaultApi.md#create_lobby_message) | **POST** /lobbies/{lobby_id}/messages | 
[**create_message**](DefaultApi.md#create_message) | **POST** /channels/{channel_id}/messages | 
[**create_or_join_lobby**](DefaultApi.md#create_or_join_lobby) | **PUT** /lobbies | 
[**create_pin**](DefaultApi.md#create_pin) | **PUT** /channels/{channel_id}/messages/pins/{message_id} | 
[**create_stage_instance**](DefaultApi.md#create_stage_instance) | **POST** /stage-instances | 
[**create_thread**](DefaultApi.md#create_thread) | **POST** /channels/{channel_id}/threads | 
[**create_thread_from_message**](DefaultApi.md#create_thread_from_message) | **POST** /channels/{channel_id}/messages/{message_id}/threads | 
[**create_webhook**](DefaultApi.md#create_webhook) | **POST** /channels/{channel_id}/webhooks | 
[**crosspost_message**](DefaultApi.md#crosspost_message) | **POST** /channels/{channel_id}/messages/{message_id}/crosspost | 
[**delete_all_message_reactions**](DefaultApi.md#delete_all_message_reactions) | **DELETE** /channels/{channel_id}/messages/{message_id}/reactions | 
[**delete_all_message_reactions_by_emoji**](DefaultApi.md#delete_all_message_reactions_by_emoji) | **DELETE** /channels/{channel_id}/messages/{message_id}/reactions/{emoji_name} | 
[**delete_application_command**](DefaultApi.md#delete_application_command) | **DELETE** /applications/{application_id}/commands/{command_id} | 
[**delete_application_emoji**](DefaultApi.md#delete_application_emoji) | **DELETE** /applications/{application_id}/emojis/{emoji_id} | 
[**delete_application_user_role_connection**](DefaultApi.md#delete_application_user_role_connection) | **DELETE** /users/@me/applications/{application_id}/role-connection | 
[**delete_auto_moderation_rule**](DefaultApi.md#delete_auto_moderation_rule) | **DELETE** /guilds/{guild_id}/auto-moderation/rules/{rule_id} | 
[**delete_channel**](DefaultApi.md#delete_channel) | **DELETE** /channels/{channel_id} | 
[**delete_channel_permission_overwrite**](DefaultApi.md#delete_channel_permission_overwrite) | **DELETE** /channels/{channel_id}/permissions/{overwrite_id} | 
[**delete_entitlement**](DefaultApi.md#delete_entitlement) | **DELETE** /applications/{application_id}/entitlements/{entitlement_id} | 
[**delete_group_dm_user**](DefaultApi.md#delete_group_dm_user) | **DELETE** /channels/{channel_id}/recipients/{user_id} | 
[**delete_guild_application_command**](DefaultApi.md#delete_guild_application_command) | **DELETE** /applications/{application_id}/guilds/{guild_id}/commands/{command_id} | 
[**delete_guild_emoji**](DefaultApi.md#delete_guild_emoji) | **DELETE** /guilds/{guild_id}/emojis/{emoji_id} | 
[**delete_guild_integration**](DefaultApi.md#delete_guild_integration) | **DELETE** /guilds/{guild_id}/integrations/{integration_id} | 
[**delete_guild_member**](DefaultApi.md#delete_guild_member) | **DELETE** /guilds/{guild_id}/members/{user_id} | 
[**delete_guild_member_role**](DefaultApi.md#delete_guild_member_role) | **DELETE** /guilds/{guild_id}/members/{user_id}/roles/{role_id} | 
[**delete_guild_role**](DefaultApi.md#delete_guild_role) | **DELETE** /guilds/{guild_id}/roles/{role_id} | 
[**delete_guild_scheduled_event**](DefaultApi.md#delete_guild_scheduled_event) | **DELETE** /guilds/{guild_id}/scheduled-events/{guild_scheduled_event_id} | 
[**delete_guild_soundboard_sound**](DefaultApi.md#delete_guild_soundboard_sound) | **DELETE** /guilds/{guild_id}/soundboard-sounds/{sound_id} | 
[**delete_guild_sticker**](DefaultApi.md#delete_guild_sticker) | **DELETE** /guilds/{guild_id}/stickers/{sticker_id} | 
[**delete_guild_template**](DefaultApi.md#delete_guild_template) | **DELETE** /guilds/{guild_id}/templates/{code} | 
[**delete_lobby_member**](DefaultApi.md#delete_lobby_member) | **DELETE** /lobbies/{lobby_id}/members/{user_id} | 
[**delete_message**](DefaultApi.md#delete_message) | **DELETE** /channels/{channel_id}/messages/{message_id} | 
[**delete_my_message_reaction**](DefaultApi.md#delete_my_message_reaction) | **DELETE** /channels/{channel_id}/messages/{message_id}/reactions/{emoji_name}/@me | 
[**delete_original_webhook_message**](DefaultApi.md#delete_original_webhook_message) | **DELETE** /webhooks/{webhook_id}/{webhook_token}/messages/@original | 
[**delete_pin**](DefaultApi.md#delete_pin) | **DELETE** /channels/{channel_id}/messages/pins/{message_id} | 
[**delete_stage_instance**](DefaultApi.md#delete_stage_instance) | **DELETE** /stage-instances/{channel_id} | 
[**delete_thread_member**](DefaultApi.md#delete_thread_member) | **DELETE** /channels/{channel_id}/thread-members/{user_id} | 
[**delete_user_message_reaction**](DefaultApi.md#delete_user_message_reaction) | **DELETE** /channels/{channel_id}/messages/{message_id}/reactions/{emoji_name}/{user_id} | 
[**delete_webhook**](DefaultApi.md#delete_webhook) | **DELETE** /webhooks/{webhook_id} | 
[**delete_webhook_by_token**](DefaultApi.md#delete_webhook_by_token) | **DELETE** /webhooks/{webhook_id}/{webhook_token} | 
[**delete_webhook_message**](DefaultApi.md#delete_webhook_message) | **DELETE** /webhooks/{webhook_id}/{webhook_token}/messages/{message_id} | 
[**deprecated_create_pin**](DefaultApi.md#deprecated_create_pin) | **PUT** /channels/{channel_id}/pins/{message_id} | 
[**deprecated_delete_pin**](DefaultApi.md#deprecated_delete_pin) | **DELETE** /channels/{channel_id}/pins/{message_id} | 
[**deprecated_list_pins**](DefaultApi.md#deprecated_list_pins) | **GET** /channels/{channel_id}/pins | 
[**edit_lobby**](DefaultApi.md#edit_lobby) | **PATCH** /lobbies/{lobby_id} | 
[**edit_lobby_channel_link**](DefaultApi.md#edit_lobby_channel_link) | **PATCH** /lobbies/{lobby_id}/channel-linking | 
[**execute_github_compatible_webhook**](DefaultApi.md#execute_github_compatible_webhook) | **POST** /webhooks/{webhook_id}/{webhook_token}/github | 
[**execute_slack_compatible_webhook**](DefaultApi.md#execute_slack_compatible_webhook) | **POST** /webhooks/{webhook_id}/{webhook_token}/slack | 
[**execute_webhook**](DefaultApi.md#execute_webhook) | **POST** /webhooks/{webhook_id}/{webhook_token} | 
[**follow_channel**](DefaultApi.md#follow_channel) | **POST** /channels/{channel_id}/followers | 
[**get_active_guild_threads**](DefaultApi.md#get_active_guild_threads) | **GET** /guilds/{guild_id}/threads/active | 
[**get_answer_voters**](DefaultApi.md#get_answer_voters) | **GET** /channels/{channel_id}/polls/{message_id}/answers/{answer_id} | 
[**get_application**](DefaultApi.md#get_application) | **GET** /applications/{application_id} | 
[**get_application_command**](DefaultApi.md#get_application_command) | **GET** /applications/{application_id}/commands/{command_id} | 
[**get_application_emoji**](DefaultApi.md#get_application_emoji) | **GET** /applications/{application_id}/emojis/{emoji_id} | 
[**get_application_role_connections_metadata**](DefaultApi.md#get_application_role_connections_metadata) | **GET** /applications/{application_id}/role-connections/metadata | 
[**get_application_user_role_connection**](DefaultApi.md#get_application_user_role_connection) | **GET** /users/@me/applications/{application_id}/role-connection | 
[**get_auto_moderation_rule**](DefaultApi.md#get_auto_moderation_rule) | **GET** /guilds/{guild_id}/auto-moderation/rules/{rule_id} | 
[**get_bot_gateway**](DefaultApi.md#get_bot_gateway) | **GET** /gateway/bot | 
[**get_channel**](DefaultApi.md#get_channel) | **GET** /channels/{channel_id} | 
[**get_entitlement**](DefaultApi.md#get_entitlement) | **GET** /applications/{application_id}/entitlements/{entitlement_id} | 
[**get_entitlements**](DefaultApi.md#get_entitlements) | **GET** /applications/{application_id}/entitlements | 
[**get_gateway**](DefaultApi.md#get_gateway) | **GET** /gateway | 
[**get_guild**](DefaultApi.md#get_guild) | **GET** /guilds/{guild_id} | 
[**get_guild_application_command**](DefaultApi.md#get_guild_application_command) | **GET** /applications/{application_id}/guilds/{guild_id}/commands/{command_id} | 
[**get_guild_application_command_permissions**](DefaultApi.md#get_guild_application_command_permissions) | **GET** /applications/{application_id}/guilds/{guild_id}/commands/{command_id}/permissions | 
[**get_guild_ban**](DefaultApi.md#get_guild_ban) | **GET** /guilds/{guild_id}/bans/{user_id} | 
[**get_guild_emoji**](DefaultApi.md#get_guild_emoji) | **GET** /guilds/{guild_id}/emojis/{emoji_id} | 
[**get_guild_member**](DefaultApi.md#get_guild_member) | **GET** /guilds/{guild_id}/members/{user_id} | 
[**get_guild_new_member_welcome**](DefaultApi.md#get_guild_new_member_welcome) | **GET** /guilds/{guild_id}/new-member-welcome | 
[**get_guild_preview**](DefaultApi.md#get_guild_preview) | **GET** /guilds/{guild_id}/preview | 
[**get_guild_role**](DefaultApi.md#get_guild_role) | **GET** /guilds/{guild_id}/roles/{role_id} | 
[**get_guild_scheduled_event**](DefaultApi.md#get_guild_scheduled_event) | **GET** /guilds/{guild_id}/scheduled-events/{guild_scheduled_event_id} | 
[**get_guild_soundboard_sound**](DefaultApi.md#get_guild_soundboard_sound) | **GET** /guilds/{guild_id}/soundboard-sounds/{sound_id} | 
[**get_guild_sticker**](DefaultApi.md#get_guild_sticker) | **GET** /guilds/{guild_id}/stickers/{sticker_id} | 
[**get_guild_template**](DefaultApi.md#get_guild_template) | **GET** /guilds/templates/{code} | 
[**get_guild_vanity_url**](DefaultApi.md#get_guild_vanity_url) | **GET** /guilds/{guild_id}/vanity-url | 
[**get_guild_webhooks**](DefaultApi.md#get_guild_webhooks) | **GET** /guilds/{guild_id}/webhooks | 
[**get_guild_welcome_screen**](DefaultApi.md#get_guild_welcome_screen) | **GET** /guilds/{guild_id}/welcome-screen | 
[**get_guild_widget**](DefaultApi.md#get_guild_widget) | **GET** /guilds/{guild_id}/widget.json | 
[**get_guild_widget_png**](DefaultApi.md#get_guild_widget_png) | **GET** /guilds/{guild_id}/widget.png | 
[**get_guild_widget_settings**](DefaultApi.md#get_guild_widget_settings) | **GET** /guilds/{guild_id}/widget | 
[**get_guilds_onboarding**](DefaultApi.md#get_guilds_onboarding) | **GET** /guilds/{guild_id}/onboarding | 
[**get_lobby**](DefaultApi.md#get_lobby) | **GET** /lobbies/{lobby_id} | 
[**get_lobby_messages**](DefaultApi.md#get_lobby_messages) | **GET** /lobbies/{lobby_id}/messages | 
[**get_message**](DefaultApi.md#get_message) | **GET** /channels/{channel_id}/messages/{message_id} | 
[**get_my_application**](DefaultApi.md#get_my_application) | **GET** /applications/@me | 
[**get_my_guild_member**](DefaultApi.md#get_my_guild_member) | **GET** /users/@me/guilds/{guild_id}/member | 
[**get_my_oauth2_application**](DefaultApi.md#get_my_oauth2_application) | **GET** /oauth2/applications/@me | 
[**get_my_oauth2_authorization**](DefaultApi.md#get_my_oauth2_authorization) | **GET** /oauth2/@me | 
[**get_my_user**](DefaultApi.md#get_my_user) | **GET** /users/@me | 
[**get_openid_connect_userinfo**](DefaultApi.md#get_openid_connect_userinfo) | **GET** /oauth2/userinfo | 
[**get_original_webhook_message**](DefaultApi.md#get_original_webhook_message) | **GET** /webhooks/{webhook_id}/{webhook_token}/messages/@original | 
[**get_public_keys**](DefaultApi.md#get_public_keys) | **GET** /oauth2/keys | 
[**get_self_voice_state**](DefaultApi.md#get_self_voice_state) | **GET** /guilds/{guild_id}/voice-states/@me | 
[**get_soundboard_default_sounds**](DefaultApi.md#get_soundboard_default_sounds) | **GET** /soundboard-default-sounds | 
[**get_stage_instance**](DefaultApi.md#get_stage_instance) | **GET** /stage-instances/{channel_id} | 
[**get_sticker**](DefaultApi.md#get_sticker) | **GET** /stickers/{sticker_id} | 
[**get_sticker_pack**](DefaultApi.md#get_sticker_pack) | **GET** /sticker-packs/{pack_id} | 
[**get_thread_member**](DefaultApi.md#get_thread_member) | **GET** /channels/{channel_id}/thread-members/{user_id} | 
[**get_user**](DefaultApi.md#get_user) | **GET** /users/{user_id} | 
[**get_voice_state**](DefaultApi.md#get_voice_state) | **GET** /guilds/{guild_id}/voice-states/{user_id} | 
[**get_webhook**](DefaultApi.md#get_webhook) | **GET** /webhooks/{webhook_id} | 
[**get_webhook_by_token**](DefaultApi.md#get_webhook_by_token) | **GET** /webhooks/{webhook_id}/{webhook_token} | 
[**get_webhook_message**](DefaultApi.md#get_webhook_message) | **GET** /webhooks/{webhook_id}/{webhook_token}/messages/{message_id} | 
[**invite_resolve**](DefaultApi.md#invite_resolve) | **GET** /invites/{code} | 
[**invite_revoke**](DefaultApi.md#invite_revoke) | **DELETE** /invites/{code} | 
[**join_thread**](DefaultApi.md#join_thread) | **PUT** /channels/{channel_id}/thread-members/@me | 
[**leave_guild**](DefaultApi.md#leave_guild) | **DELETE** /users/@me/guilds/{guild_id} | 
[**leave_lobby**](DefaultApi.md#leave_lobby) | **DELETE** /lobbies/{lobby_id}/members/@me | 
[**leave_thread**](DefaultApi.md#leave_thread) | **DELETE** /channels/{channel_id}/thread-members/@me | 
[**list_application_commands**](DefaultApi.md#list_application_commands) | **GET** /applications/{application_id}/commands | 
[**list_application_emojis**](DefaultApi.md#list_application_emojis) | **GET** /applications/{application_id}/emojis | 
[**list_auto_moderation_rules**](DefaultApi.md#list_auto_moderation_rules) | **GET** /guilds/{guild_id}/auto-moderation/rules | 
[**list_channel_invites**](DefaultApi.md#list_channel_invites) | **GET** /channels/{channel_id}/invites | 
[**list_channel_webhooks**](DefaultApi.md#list_channel_webhooks) | **GET** /channels/{channel_id}/webhooks | 
[**list_guild_application_command_permissions**](DefaultApi.md#list_guild_application_command_permissions) | **GET** /applications/{application_id}/guilds/{guild_id}/commands/permissions | 
[**list_guild_application_commands**](DefaultApi.md#list_guild_application_commands) | **GET** /applications/{application_id}/guilds/{guild_id}/commands | 
[**list_guild_audit_log_entries**](DefaultApi.md#list_guild_audit_log_entries) | **GET** /guilds/{guild_id}/audit-logs | 
[**list_guild_bans**](DefaultApi.md#list_guild_bans) | **GET** /guilds/{guild_id}/bans | 
[**list_guild_channels**](DefaultApi.md#list_guild_channels) | **GET** /guilds/{guild_id}/channels | 
[**list_guild_emojis**](DefaultApi.md#list_guild_emojis) | **GET** /guilds/{guild_id}/emojis | 
[**list_guild_integrations**](DefaultApi.md#list_guild_integrations) | **GET** /guilds/{guild_id}/integrations | 
[**list_guild_invites**](DefaultApi.md#list_guild_invites) | **GET** /guilds/{guild_id}/invites | 
[**list_guild_members**](DefaultApi.md#list_guild_members) | **GET** /guilds/{guild_id}/members | 
[**list_guild_roles**](DefaultApi.md#list_guild_roles) | **GET** /guilds/{guild_id}/roles | 
[**list_guild_scheduled_event_users**](DefaultApi.md#list_guild_scheduled_event_users) | **GET** /guilds/{guild_id}/scheduled-events/{guild_scheduled_event_id}/users | 
[**list_guild_scheduled_events**](DefaultApi.md#list_guild_scheduled_events) | **GET** /guilds/{guild_id}/scheduled-events | 
[**list_guild_soundboard_sounds**](DefaultApi.md#list_guild_soundboard_sounds) | **GET** /guilds/{guild_id}/soundboard-sounds | 
[**list_guild_stickers**](DefaultApi.md#list_guild_stickers) | **GET** /guilds/{guild_id}/stickers | 
[**list_guild_templates**](DefaultApi.md#list_guild_templates) | **GET** /guilds/{guild_id}/templates | 
[**list_guild_voice_regions**](DefaultApi.md#list_guild_voice_regions) | **GET** /guilds/{guild_id}/regions | 
[**list_message_reactions_by_emoji**](DefaultApi.md#list_message_reactions_by_emoji) | **GET** /channels/{channel_id}/messages/{message_id}/reactions/{emoji_name} | 
[**list_messages**](DefaultApi.md#list_messages) | **GET** /channels/{channel_id}/messages | 
[**list_my_connections**](DefaultApi.md#list_my_connections) | **GET** /users/@me/connections | 
[**list_my_guilds**](DefaultApi.md#list_my_guilds) | **GET** /users/@me/guilds | 
[**list_my_private_archived_threads**](DefaultApi.md#list_my_private_archived_threads) | **GET** /channels/{channel_id}/users/@me/threads/archived/private | 
[**list_pins**](DefaultApi.md#list_pins) | **GET** /channels/{channel_id}/messages/pins | 
[**list_private_archived_threads**](DefaultApi.md#list_private_archived_threads) | **GET** /channels/{channel_id}/threads/archived/private | 
[**list_public_archived_threads**](DefaultApi.md#list_public_archived_threads) | **GET** /channels/{channel_id}/threads/archived/public | 
[**list_sticker_packs**](DefaultApi.md#list_sticker_packs) | **GET** /sticker-packs | 
[**list_thread_members**](DefaultApi.md#list_thread_members) | **GET** /channels/{channel_id}/thread-members | 
[**list_voice_regions**](DefaultApi.md#list_voice_regions) | **GET** /voice/regions | 
[**partner_sdk_token**](DefaultApi.md#partner_sdk_token) | **POST** /partner-sdk/token | 
[**partner_sdk_unmerge_provisional_account**](DefaultApi.md#partner_sdk_unmerge_provisional_account) | **POST** /partner-sdk/provisional-accounts/unmerge | 
[**poll_expire**](DefaultApi.md#poll_expire) | **POST** /channels/{channel_id}/polls/{message_id}/expire | 
[**preview_prune_guild**](DefaultApi.md#preview_prune_guild) | **GET** /guilds/{guild_id}/prune | 
[**prune_guild**](DefaultApi.md#prune_guild) | **POST** /guilds/{guild_id}/prune | 
[**put_guilds_onboarding**](DefaultApi.md#put_guilds_onboarding) | **PUT** /guilds/{guild_id}/onboarding | 
[**search_guild_members**](DefaultApi.md#search_guild_members) | **GET** /guilds/{guild_id}/members/search | 
[**send_soundboard_sound**](DefaultApi.md#send_soundboard_sound) | **POST** /channels/{channel_id}/send-soundboard-sound | 
[**set_channel_permission_overwrite**](DefaultApi.md#set_channel_permission_overwrite) | **PUT** /channels/{channel_id}/permissions/{overwrite_id} | 
[**set_guild_application_command_permissions**](DefaultApi.md#set_guild_application_command_permissions) | **PUT** /applications/{application_id}/guilds/{guild_id}/commands/{command_id}/permissions | 
[**sync_guild_template**](DefaultApi.md#sync_guild_template) | **PUT** /guilds/{guild_id}/templates/{code} | 
[**thread_search**](DefaultApi.md#thread_search) | **GET** /channels/{channel_id}/threads/search | 
[**trigger_typing_indicator**](DefaultApi.md#trigger_typing_indicator) | **POST** /channels/{channel_id}/typing | 
[**unban_user_from_guild**](DefaultApi.md#unban_user_from_guild) | **DELETE** /guilds/{guild_id}/bans/{user_id} | 
[**update_application**](DefaultApi.md#update_application) | **PATCH** /applications/{application_id} | 
[**update_application_command**](DefaultApi.md#update_application_command) | **PATCH** /applications/{application_id}/commands/{command_id} | 
[**update_application_emoji**](DefaultApi.md#update_application_emoji) | **PATCH** /applications/{application_id}/emojis/{emoji_id} | 
[**update_application_role_connections_metadata**](DefaultApi.md#update_application_role_connections_metadata) | **PUT** /applications/{application_id}/role-connections/metadata | 
[**update_application_user_role_connection**](DefaultApi.md#update_application_user_role_connection) | **PUT** /users/@me/applications/{application_id}/role-connection | 
[**update_auto_moderation_rule**](DefaultApi.md#update_auto_moderation_rule) | **PATCH** /guilds/{guild_id}/auto-moderation/rules/{rule_id} | 
[**update_channel**](DefaultApi.md#update_channel) | **PATCH** /channels/{channel_id} | 
[**update_guild**](DefaultApi.md#update_guild) | **PATCH** /guilds/{guild_id} | 
[**update_guild_application_command**](DefaultApi.md#update_guild_application_command) | **PATCH** /applications/{application_id}/guilds/{guild_id}/commands/{command_id} | 
[**update_guild_emoji**](DefaultApi.md#update_guild_emoji) | **PATCH** /guilds/{guild_id}/emojis/{emoji_id} | 
[**update_guild_member**](DefaultApi.md#update_guild_member) | **PATCH** /guilds/{guild_id}/members/{user_id} | 
[**update_guild_role**](DefaultApi.md#update_guild_role) | **PATCH** /guilds/{guild_id}/roles/{role_id} | 
[**update_guild_scheduled_event**](DefaultApi.md#update_guild_scheduled_event) | **PATCH** /guilds/{guild_id}/scheduled-events/{guild_scheduled_event_id} | 
[**update_guild_soundboard_sound**](DefaultApi.md#update_guild_soundboard_sound) | **PATCH** /guilds/{guild_id}/soundboard-sounds/{sound_id} | 
[**update_guild_sticker**](DefaultApi.md#update_guild_sticker) | **PATCH** /guilds/{guild_id}/stickers/{sticker_id} | 
[**update_guild_template**](DefaultApi.md#update_guild_template) | **PATCH** /guilds/{guild_id}/templates/{code} | 
[**update_guild_welcome_screen**](DefaultApi.md#update_guild_welcome_screen) | **PATCH** /guilds/{guild_id}/welcome-screen | 
[**update_guild_widget_settings**](DefaultApi.md#update_guild_widget_settings) | **PATCH** /guilds/{guild_id}/widget | 
[**update_message**](DefaultApi.md#update_message) | **PATCH** /channels/{channel_id}/messages/{message_id} | 
[**update_my_application**](DefaultApi.md#update_my_application) | **PATCH** /applications/@me | 
[**update_my_guild_member**](DefaultApi.md#update_my_guild_member) | **PATCH** /guilds/{guild_id}/members/@me | 
[**update_my_user**](DefaultApi.md#update_my_user) | **PATCH** /users/@me | 
[**update_original_webhook_message**](DefaultApi.md#update_original_webhook_message) | **PATCH** /webhooks/{webhook_id}/{webhook_token}/messages/@original | 
[**update_self_voice_state**](DefaultApi.md#update_self_voice_state) | **PATCH** /guilds/{guild_id}/voice-states/@me | 
[**update_stage_instance**](DefaultApi.md#update_stage_instance) | **PATCH** /stage-instances/{channel_id} | 
[**update_voice_state**](DefaultApi.md#update_voice_state) | **PATCH** /guilds/{guild_id}/voice-states/{user_id} | 
[**update_webhook**](DefaultApi.md#update_webhook) | **PATCH** /webhooks/{webhook_id} | 
[**update_webhook_by_token**](DefaultApi.md#update_webhook_by_token) | **PATCH** /webhooks/{webhook_id}/{webhook_token} | 
[**update_webhook_message**](DefaultApi.md#update_webhook_message) | **PATCH** /webhooks/{webhook_id}/{webhook_token}/messages/{message_id} | 
[**upload_application_attachment**](DefaultApi.md#upload_application_attachment) | **POST** /applications/{application_id}/attachment | 



## add_group_dm_user

> models::AddGroupDmUser201Response add_group_dm_user(channel_id, user_id, add_group_dm_user_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |
**add_group_dm_user_request** | [**AddGroupDmUserRequest**](AddGroupDmUserRequest.md) |  | [required] |

### Return type

[**models::AddGroupDmUser201Response**](add_group_dm_user_201_response.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_guild_member

> models::GuildMemberResponse add_guild_member(guild_id, user_id, bot_add_guild_member_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |
**bot_add_guild_member_request** | [**BotAddGuildMemberRequest**](BotAddGuildMemberRequest.md) |  | [required] |

### Return type

[**models::GuildMemberResponse**](GuildMemberResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_guild_member_role

> add_guild_member_role(guild_id, user_id, role_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |
**role_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_lobby_member

> models::LobbyMemberResponse add_lobby_member(lobby_id, user_id, add_lobby_member_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lobby_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |
**add_lobby_member_request** | [**AddLobbyMemberRequest**](AddLobbyMemberRequest.md) |  | [required] |

### Return type

[**models::LobbyMemberResponse**](LobbyMemberResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_my_message_reaction

> add_my_message_reaction(channel_id, message_id, emoji_name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**message_id** | **String** |  | [required] |
**emoji_name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_thread_member

> add_thread_member(channel_id, user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applications_get_activity_instance

> models::EmbeddedActivityInstance applications_get_activity_instance(application_id, instance_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** |  | [required] |
**instance_id** | **String** |  | [required] |

### Return type

[**models::EmbeddedActivityInstance**](EmbeddedActivityInstance.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ban_user_from_guild

> ban_user_from_guild(guild_id, user_id, ban_user_from_guild_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |
**ban_user_from_guild_request** | [**BanUserFromGuildRequest**](BanUserFromGuildRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bot_partner_sdk_token

> models::ProvisionalTokenResponse bot_partner_sdk_token(bot_partner_sdk_token_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_partner_sdk_token_request** | [**BotPartnerSdkTokenRequest**](BotPartnerSdkTokenRequest.md) |  | [required] |

### Return type

[**models::ProvisionalTokenResponse**](ProvisionalTokenResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bot_partner_sdk_unmerge_provisional_account

> bot_partner_sdk_unmerge_provisional_account(bot_partner_sdk_unmerge_provisional_account_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_partner_sdk_unmerge_provisional_account_request** | [**BotPartnerSdkUnmergeProvisionalAccountRequest**](BotPartnerSdkUnmergeProvisionalAccountRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_ban_users_from_guild

> models::BulkBanUsersResponse bulk_ban_users_from_guild(guild_id, bulk_ban_users_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**bulk_ban_users_request** | [**BulkBanUsersRequest**](BulkBanUsersRequest.md) |  | [required] |

### Return type

[**models::BulkBanUsersResponse**](BulkBanUsersResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_delete_messages

> bulk_delete_messages(channel_id, bulk_delete_messages_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**bulk_delete_messages_request** | [**BulkDeleteMessagesRequest**](BulkDeleteMessagesRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_set_application_commands

> Vec<models::ApplicationCommandResponse> bulk_set_application_commands(application_id, application_command_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** |  | [required] |
**application_command_update_request** | Option<[**Vec<models::ApplicationCommandUpdateRequest>**](ApplicationCommandUpdateRequest.md)> |  | [required] |

### Return type

[**Vec<models::ApplicationCommandResponse>**](ApplicationCommandResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_set_guild_application_commands

> Vec<models::ApplicationCommandResponse> bulk_set_guild_application_commands(application_id, guild_id, application_command_update_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** |  | [required] |
**guild_id** | **String** |  | [required] |
**application_command_update_request** | Option<[**Vec<models::ApplicationCommandUpdateRequest>**](ApplicationCommandUpdateRequest.md)> |  | [required] |

### Return type

[**Vec<models::ApplicationCommandResponse>**](ApplicationCommandResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_update_guild_channels

> bulk_update_guild_channels(guild_id, bulk_update_guild_channels_request_inner)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**bulk_update_guild_channels_request_inner** | [**Vec<models::BulkUpdateGuildChannelsRequestInner>**](bulk_update_guild_channels_request_inner.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_update_guild_roles

> Vec<models::GuildRoleResponse> bulk_update_guild_roles(guild_id, update_role_positions_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**update_role_positions_request** | [**Vec<models::UpdateRolePositionsRequest>**](UpdateRolePositionsRequest.md) |  | [required] |

### Return type

[**Vec<models::GuildRoleResponse>**](GuildRoleResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_update_lobby_members

> Vec<models::LobbyMemberResponse> bulk_update_lobby_members(lobby_id, bulk_lobby_member_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lobby_id** | **String** |  | [required] |
**bulk_lobby_member_request** | Option<[**Vec<models::BulkLobbyMemberRequest>**](BulkLobbyMemberRequest.md)> |  | [required] |

### Return type

[**Vec<models::LobbyMemberResponse>**](LobbyMemberResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## consume_entitlement

> consume_entitlement(application_id, entitlement_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** |  | [required] |
**entitlement_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_application_command

> models::ApplicationCommandResponse create_application_command(application_id, application_command_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** |  | [required] |
**application_command_create_request** | [**ApplicationCommandCreateRequest**](ApplicationCommandCreateRequest.md) |  | [required] |

### Return type

[**models::ApplicationCommandResponse**](ApplicationCommandResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_application_emoji

> models::EmojiResponse create_application_emoji(application_id, create_application_emoji_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** |  | [required] |
**create_application_emoji_request** | [**CreateApplicationEmojiRequest**](CreateApplicationEmojiRequest.md) |  | [required] |

### Return type

[**models::EmojiResponse**](EmojiResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_auto_moderation_rule

> models::CreateAutoModerationRule200Response create_auto_moderation_rule(guild_id, create_auto_moderation_rule_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**create_auto_moderation_rule_request** | [**CreateAutoModerationRuleRequest**](CreateAutoModerationRuleRequest.md) |  | [required] |

### Return type

[**models::CreateAutoModerationRule200Response**](create_auto_moderation_rule_200_response.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_channel_invite

> models::CreateChannelInvite200Response create_channel_invite(channel_id, create_channel_invite_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**create_channel_invite_request** | [**CreateChannelInviteRequest**](CreateChannelInviteRequest.md) |  | [required] |

### Return type

[**models::CreateChannelInvite200Response**](create_channel_invite_200_response.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dm

> models::AddGroupDmUser201Response create_dm(create_private_channel_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_private_channel_request** | [**CreatePrivateChannelRequest**](CreatePrivateChannelRequest.md) |  | [required] |

### Return type

[**models::AddGroupDmUser201Response**](add_group_dm_user_201_response.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_entitlement

> models::EntitlementResponse create_entitlement(application_id, create_entitlement_request_data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** |  | [required] |
**create_entitlement_request_data** | [**CreateEntitlementRequestData**](CreateEntitlementRequestData.md) |  | [required] |

### Return type

[**models::EntitlementResponse**](EntitlementResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_guild_application_command

> models::ApplicationCommandResponse create_guild_application_command(application_id, guild_id, application_command_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** |  | [required] |
**guild_id** | **String** |  | [required] |
**application_command_create_request** | [**ApplicationCommandCreateRequest**](ApplicationCommandCreateRequest.md) |  | [required] |

### Return type

[**models::ApplicationCommandResponse**](ApplicationCommandResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_guild_channel

> models::GuildChannelResponse create_guild_channel(guild_id, create_guild_channel_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**create_guild_channel_request** | [**CreateGuildChannelRequest**](CreateGuildChannelRequest.md) |  | [required] |

### Return type

[**models::GuildChannelResponse**](GuildChannelResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_guild_emoji

> models::EmojiResponse create_guild_emoji(guild_id, create_guild_emoji_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**create_guild_emoji_request** | [**CreateGuildEmojiRequest**](CreateGuildEmojiRequest.md) |  | [required] |

### Return type

[**models::EmojiResponse**](EmojiResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_guild_role

> models::GuildRoleResponse create_guild_role(guild_id, create_role_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**create_role_request** | [**CreateRoleRequest**](CreateRoleRequest.md) |  | [required] |

### Return type

[**models::GuildRoleResponse**](GuildRoleResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_guild_scheduled_event

> models::ListGuildScheduledEvents200ResponseInner create_guild_scheduled_event(guild_id, create_guild_scheduled_event_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**create_guild_scheduled_event_request** | [**CreateGuildScheduledEventRequest**](CreateGuildScheduledEventRequest.md) |  | [required] |

### Return type

[**models::ListGuildScheduledEvents200ResponseInner**](list_guild_scheduled_events_200_response_inner.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_guild_soundboard_sound

> models::SoundboardSoundResponse create_guild_soundboard_sound(guild_id, soundboard_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**soundboard_create_request** | [**SoundboardCreateRequest**](SoundboardCreateRequest.md) |  | [required] |

### Return type

[**models::SoundboardSoundResponse**](SoundboardSoundResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_guild_sticker

> models::GuildStickerResponse create_guild_sticker(guild_id, name, tags, file, description)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**name** | **String** |  | [required] |
**tags** | **String** |  | [required] |
**file** | **String** |  | [required] |
**description** | Option<**String**> |  |  |

### Return type

[**models::GuildStickerResponse**](GuildStickerResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_guild_template

> models::GuildTemplateResponse create_guild_template(guild_id, create_guild_template_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**create_guild_template_request** | [**CreateGuildTemplateRequest**](CreateGuildTemplateRequest.md) |  | [required] |

### Return type

[**models::GuildTemplateResponse**](GuildTemplateResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_interaction_response

> models::InteractionCallbackResponse create_interaction_response(interaction_id, interaction_token, create_interaction_response_request, with_response)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**interaction_id** | **String** |  | [required] |
**interaction_token** | **String** |  | [required] |
**create_interaction_response_request** | [**CreateInteractionResponseRequest**](CreateInteractionResponseRequest.md) |  | [required] |
**with_response** | Option<**bool**> |  |  |

### Return type

[**models::InteractionCallbackResponse**](InteractionCallbackResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_linked_lobby_guild_invite_for_self

> models::LobbyGuildInviteResponse create_linked_lobby_guild_invite_for_self(lobby_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lobby_id** | **String** |  | [required] |

### Return type

[**models::LobbyGuildInviteResponse**](LobbyGuildInviteResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_linked_lobby_guild_invite_for_user

> models::LobbyGuildInviteResponse create_linked_lobby_guild_invite_for_user(lobby_id, user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lobby_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

[**models::LobbyGuildInviteResponse**](LobbyGuildInviteResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_lobby

> models::LobbyResponse create_lobby(create_lobby_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_lobby_request** | [**CreateLobbyRequest**](CreateLobbyRequest.md) |  | [required] |

### Return type

[**models::LobbyResponse**](LobbyResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_lobby_message

> models::LobbyMessageResponse create_lobby_message(lobby_id, sdk_message_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lobby_id** | **String** |  | [required] |
**sdk_message_request** | [**SdkMessageRequest**](SdkMessageRequest.md) |  | [required] |

### Return type

[**models::LobbyMessageResponse**](LobbyMessageResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_message

> models::MessageResponse create_message(channel_id, message_create_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**message_create_request** | [**MessageCreateRequest**](MessageCreateRequest.md) |  | [required] |

### Return type

[**models::MessageResponse**](MessageResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_or_join_lobby

> models::LobbyResponse create_or_join_lobby(create_or_join_lobby_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_or_join_lobby_request** | [**CreateOrJoinLobbyRequest**](CreateOrJoinLobbyRequest.md) |  | [required] |

### Return type

[**models::LobbyResponse**](LobbyResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_pin

> create_pin(channel_id, message_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**message_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_stage_instance

> models::StageInstanceResponse create_stage_instance(create_stage_instance_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_stage_instance_request** | [**CreateStageInstanceRequest**](CreateStageInstanceRequest.md) |  | [required] |

### Return type

[**models::StageInstanceResponse**](StageInstanceResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_thread

> models::CreatedThreadResponse create_thread(channel_id, create_thread_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**create_thread_request** | [**CreateThreadRequest**](CreateThreadRequest.md) |  | [required] |

### Return type

[**models::CreatedThreadResponse**](CreatedThreadResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_thread_from_message

> models::ThreadResponse create_thread_from_message(channel_id, message_id, create_text_thread_with_message_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**message_id** | **String** |  | [required] |
**create_text_thread_with_message_request** | [**CreateTextThreadWithMessageRequest**](CreateTextThreadWithMessageRequest.md) |  | [required] |

### Return type

[**models::ThreadResponse**](ThreadResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_webhook

> models::GuildIncomingWebhookResponse create_webhook(channel_id, create_webhook_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**create_webhook_request** | [**CreateWebhookRequest**](CreateWebhookRequest.md) |  | [required] |

### Return type

[**models::GuildIncomingWebhookResponse**](GuildIncomingWebhookResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## crosspost_message

> models::MessageResponse crosspost_message(channel_id, message_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**message_id** | **String** |  | [required] |

### Return type

[**models::MessageResponse**](MessageResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_all_message_reactions

> delete_all_message_reactions(channel_id, message_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**message_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_all_message_reactions_by_emoji

> delete_all_message_reactions_by_emoji(channel_id, message_id, emoji_name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**message_id** | **String** |  | [required] |
**emoji_name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_application_command

> delete_application_command(application_id, command_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** |  | [required] |
**command_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_application_emoji

> delete_application_emoji(application_id, emoji_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** |  | [required] |
**emoji_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_application_user_role_connection

> delete_application_user_role_connection(application_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_auto_moderation_rule

> delete_auto_moderation_rule(guild_id, rule_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**rule_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_channel

> models::GetChannel200Response delete_channel(channel_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |

### Return type

[**models::GetChannel200Response**](get_channel_200_response.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_channel_permission_overwrite

> delete_channel_permission_overwrite(channel_id, overwrite_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**overwrite_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_entitlement

> delete_entitlement(application_id, entitlement_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** |  | [required] |
**entitlement_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group_dm_user

> delete_group_dm_user(channel_id, user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_guild_application_command

> delete_guild_application_command(application_id, guild_id, command_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** |  | [required] |
**guild_id** | **String** |  | [required] |
**command_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_guild_emoji

> delete_guild_emoji(guild_id, emoji_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**emoji_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_guild_integration

> delete_guild_integration(guild_id, integration_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**integration_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_guild_member

> delete_guild_member(guild_id, user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_guild_member_role

> delete_guild_member_role(guild_id, user_id, role_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |
**role_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_guild_role

> delete_guild_role(guild_id, role_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**role_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_guild_scheduled_event

> delete_guild_scheduled_event(guild_id, guild_scheduled_event_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**guild_scheduled_event_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_guild_soundboard_sound

> delete_guild_soundboard_sound(guild_id, sound_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**sound_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_guild_sticker

> delete_guild_sticker(guild_id, sticker_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**sticker_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_guild_template

> models::GuildTemplateResponse delete_guild_template(guild_id, code)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**code** | **String** |  | [required] |

### Return type

[**models::GuildTemplateResponse**](GuildTemplateResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_lobby_member

> delete_lobby_member(lobby_id, user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lobby_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_message

> delete_message(channel_id, message_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**message_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_my_message_reaction

> delete_my_message_reaction(channel_id, message_id, emoji_name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**message_id** | **String** |  | [required] |
**emoji_name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_original_webhook_message

> delete_original_webhook_message(webhook_id, webhook_token, thread_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** |  | [required] |
**webhook_token** | **String** |  | [required] |
**thread_id** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_pin

> delete_pin(channel_id, message_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**message_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_stage_instance

> delete_stage_instance(channel_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_thread_member

> delete_thread_member(channel_id, user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_message_reaction

> delete_user_message_reaction(channel_id, message_id, emoji_name, user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**message_id** | **String** |  | [required] |
**emoji_name** | **String** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_webhook

> delete_webhook(webhook_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_webhook_by_token

> delete_webhook_by_token(webhook_id, webhook_token)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** |  | [required] |
**webhook_token** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_webhook_message

> delete_webhook_message(webhook_id, webhook_token, message_id, thread_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** |  | [required] |
**webhook_token** | **String** |  | [required] |
**message_id** | **String** |  | [required] |
**thread_id** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deprecated_create_pin

> deprecated_create_pin(channel_id, message_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**message_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deprecated_delete_pin

> deprecated_delete_pin(channel_id, message_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**message_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deprecated_list_pins

> Vec<models::MessageResponse> deprecated_list_pins(channel_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |

### Return type

[**Vec<models::MessageResponse>**](MessageResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_lobby

> models::LobbyResponse edit_lobby(lobby_id, create_lobby_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lobby_id** | **String** |  | [required] |
**create_lobby_request** | [**CreateLobbyRequest**](CreateLobbyRequest.md) |  | [required] |

### Return type

[**models::LobbyResponse**](LobbyResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_lobby_channel_link

> models::LobbyResponse edit_lobby_channel_link(lobby_id, edit_lobby_channel_link_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lobby_id** | **String** |  | [required] |
**edit_lobby_channel_link_request** | [**EditLobbyChannelLinkRequest**](EditLobbyChannelLinkRequest.md) |  | [required] |

### Return type

[**models::LobbyResponse**](LobbyResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## execute_github_compatible_webhook

> execute_github_compatible_webhook(webhook_id, webhook_token, github_webhook, wait, thread_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** |  | [required] |
**webhook_token** | **String** |  | [required] |
**github_webhook** | [**GithubWebhook**](GithubWebhook.md) |  | [required] |
**wait** | Option<**bool**> |  |  |
**thread_id** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## execute_slack_compatible_webhook

> String execute_slack_compatible_webhook(webhook_id, webhook_token, slack_webhook, wait, thread_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** |  | [required] |
**webhook_token** | **String** |  | [required] |
**slack_webhook** | [**SlackWebhook**](SlackWebhook.md) |  | [required] |
**wait** | Option<**bool**> |  |  |
**thread_id** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## execute_webhook

> models::MessageResponse execute_webhook(webhook_id, webhook_token, execute_webhook_request, wait, thread_id, with_components)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** |  | [required] |
**webhook_token** | **String** |  | [required] |
**execute_webhook_request** | [**ExecuteWebhookRequest**](ExecuteWebhookRequest.md) |  | [required] |
**wait** | Option<**bool**> |  |  |
**thread_id** | Option<**String**> |  |  |
**with_components** | Option<**bool**> |  |  |

### Return type

[**models::MessageResponse**](MessageResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## follow_channel

> models::ChannelFollowerResponse follow_channel(channel_id, follow_channel_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**follow_channel_request** | [**FollowChannelRequest**](FollowChannelRequest.md) |  | [required] |

### Return type

[**models::ChannelFollowerResponse**](ChannelFollowerResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_active_guild_threads

> models::ThreadsResponse get_active_guild_threads(guild_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |

### Return type

[**models::ThreadsResponse**](ThreadsResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_answer_voters

> models::PollAnswerDetailsResponse get_answer_voters(channel_id, message_id, answer_id, after, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**message_id** | **String** |  | [required] |
**answer_id** | **i32** |  | [required] |
**after** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |

### Return type

[**models::PollAnswerDetailsResponse**](PollAnswerDetailsResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_application

> models::PrivateApplicationResponse get_application(application_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** |  | [required] |

### Return type

[**models::PrivateApplicationResponse**](PrivateApplicationResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_application_command

> models::ApplicationCommandResponse get_application_command(application_id, command_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** |  | [required] |
**command_id** | **String** |  | [required] |

### Return type

[**models::ApplicationCommandResponse**](ApplicationCommandResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_application_emoji

> models::EmojiResponse get_application_emoji(application_id, emoji_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** |  | [required] |
**emoji_id** | **String** |  | [required] |

### Return type

[**models::EmojiResponse**](EmojiResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_application_role_connections_metadata

> Vec<models::ApplicationRoleConnectionsMetadataItemResponse> get_application_role_connections_metadata(application_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** |  | [required] |

### Return type

[**Vec<models::ApplicationRoleConnectionsMetadataItemResponse>**](ApplicationRoleConnectionsMetadataItemResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_application_user_role_connection

> models::ApplicationUserRoleConnectionResponse get_application_user_role_connection(application_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** |  | [required] |

### Return type

[**models::ApplicationUserRoleConnectionResponse**](ApplicationUserRoleConnectionResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_auto_moderation_rule

> models::CreateAutoModerationRule200Response get_auto_moderation_rule(guild_id, rule_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**rule_id** | **String** |  | [required] |

### Return type

[**models::CreateAutoModerationRule200Response**](create_auto_moderation_rule_200_response.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bot_gateway

> models::GatewayBotResponse get_bot_gateway()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GatewayBotResponse**](GatewayBotResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_channel

> models::GetChannel200Response get_channel(channel_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |

### Return type

[**models::GetChannel200Response**](get_channel_200_response.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_entitlement

> models::EntitlementResponse get_entitlement(application_id, entitlement_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** |  | [required] |
**entitlement_id** | **String** |  | [required] |

### Return type

[**models::EntitlementResponse**](EntitlementResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_entitlements

> Vec<models::EntitlementResponse> get_entitlements(application_id, user_id, sku_ids, guild_id, before, after, limit, exclude_ended, exclude_deleted, only_active)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** |  | [required] |
**user_id** | Option<**String**> |  |  |
**sku_ids** | Option<**String**> |  |  |
**guild_id** | Option<**String**> |  |  |
**before** | Option<**String**> |  |  |
**after** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |
**exclude_ended** | Option<**bool**> |  |  |
**exclude_deleted** | Option<**bool**> |  |  |
**only_active** | Option<**bool**> |  |  |

### Return type

[**Vec<models::EntitlementResponse>**](EntitlementResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gateway

> models::GatewayResponse get_gateway()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GatewayResponse**](GatewayResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_guild

> models::GuildWithCountsResponse get_guild(guild_id, with_counts)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**with_counts** | Option<**bool**> |  |  |

### Return type

[**models::GuildWithCountsResponse**](GuildWithCountsResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_guild_application_command

> models::ApplicationCommandResponse get_guild_application_command(application_id, guild_id, command_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** |  | [required] |
**guild_id** | **String** |  | [required] |
**command_id** | **String** |  | [required] |

### Return type

[**models::ApplicationCommandResponse**](ApplicationCommandResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_guild_application_command_permissions

> models::CommandPermissionsResponse get_guild_application_command_permissions(application_id, guild_id, command_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** |  | [required] |
**guild_id** | **String** |  | [required] |
**command_id** | **String** |  | [required] |

### Return type

[**models::CommandPermissionsResponse**](CommandPermissionsResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_guild_ban

> models::GuildBanResponse get_guild_ban(guild_id, user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

[**models::GuildBanResponse**](GuildBanResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_guild_emoji

> models::EmojiResponse get_guild_emoji(guild_id, emoji_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**emoji_id** | **String** |  | [required] |

### Return type

[**models::EmojiResponse**](EmojiResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_guild_member

> models::GuildMemberResponse get_guild_member(guild_id, user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

[**models::GuildMemberResponse**](GuildMemberResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_guild_new_member_welcome

> models::GuildHomeSettingsResponse get_guild_new_member_welcome(guild_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |

### Return type

[**models::GuildHomeSettingsResponse**](GuildHomeSettingsResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_guild_preview

> models::GuildPreviewResponse get_guild_preview(guild_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |

### Return type

[**models::GuildPreviewResponse**](GuildPreviewResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_guild_role

> models::GuildRoleResponse get_guild_role(guild_id, role_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**role_id** | **String** |  | [required] |

### Return type

[**models::GuildRoleResponse**](GuildRoleResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_guild_scheduled_event

> models::ListGuildScheduledEvents200ResponseInner get_guild_scheduled_event(guild_id, guild_scheduled_event_id, with_user_count)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**guild_scheduled_event_id** | **String** |  | [required] |
**with_user_count** | Option<**bool**> |  |  |

### Return type

[**models::ListGuildScheduledEvents200ResponseInner**](list_guild_scheduled_events_200_response_inner.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_guild_soundboard_sound

> models::SoundboardSoundResponse get_guild_soundboard_sound(guild_id, sound_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**sound_id** | **String** |  | [required] |

### Return type

[**models::SoundboardSoundResponse**](SoundboardSoundResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_guild_sticker

> models::GuildStickerResponse get_guild_sticker(guild_id, sticker_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**sticker_id** | **String** |  | [required] |

### Return type

[**models::GuildStickerResponse**](GuildStickerResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_guild_template

> models::GuildTemplateResponse get_guild_template(code)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** |  | [required] |

### Return type

[**models::GuildTemplateResponse**](GuildTemplateResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_guild_vanity_url

> models::VanityUrlResponse get_guild_vanity_url(guild_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |

### Return type

[**models::VanityUrlResponse**](VanityURLResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_guild_webhooks

> Vec<models::ListChannelWebhooks200ResponseInner> get_guild_webhooks(guild_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |

### Return type

[**Vec<models::ListChannelWebhooks200ResponseInner>**](list_channel_webhooks_200_response_inner.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_guild_welcome_screen

> models::GuildWelcomeScreenResponse get_guild_welcome_screen(guild_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |

### Return type

[**models::GuildWelcomeScreenResponse**](GuildWelcomeScreenResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_guild_widget

> models::WidgetResponse get_guild_widget(guild_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |

### Return type

[**models::WidgetResponse**](WidgetResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_guild_widget_png

> String get_guild_widget_png(guild_id, style)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**style** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/png, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_guild_widget_settings

> models::WidgetSettingsResponse get_guild_widget_settings(guild_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |

### Return type

[**models::WidgetSettingsResponse**](WidgetSettingsResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_guilds_onboarding

> models::UserGuildOnboardingResponse get_guilds_onboarding(guild_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |

### Return type

[**models::UserGuildOnboardingResponse**](UserGuildOnboardingResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lobby

> models::LobbyResponse get_lobby(lobby_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lobby_id** | **String** |  | [required] |

### Return type

[**models::LobbyResponse**](LobbyResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lobby_messages

> Vec<models::LobbyMessageResponse> get_lobby_messages(lobby_id, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lobby_id** | **String** |  | [required] |
**limit** | Option<**i32**> |  |  |

### Return type

[**Vec<models::LobbyMessageResponse>**](LobbyMessageResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_message

> models::MessageResponse get_message(channel_id, message_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**message_id** | **String** |  | [required] |

### Return type

[**models::MessageResponse**](MessageResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_application

> models::PrivateApplicationResponse get_my_application()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PrivateApplicationResponse**](PrivateApplicationResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_guild_member

> models::PrivateGuildMemberResponse get_my_guild_member(guild_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |

### Return type

[**models::PrivateGuildMemberResponse**](PrivateGuildMemberResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_oauth2_application

> models::PrivateApplicationResponse get_my_oauth2_application()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PrivateApplicationResponse**](PrivateApplicationResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_oauth2_authorization

> models::OAuth2GetAuthorizationResponse get_my_oauth2_authorization()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::OAuth2GetAuthorizationResponse**](OAuth2GetAuthorizationResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_user

> models::UserPiiResponse get_my_user()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UserPiiResponse**](UserPIIResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_openid_connect_userinfo

> models::OAuth2GetOpenIdConnectUserInfoResponse get_openid_connect_userinfo()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::OAuth2GetOpenIdConnectUserInfoResponse**](OAuth2GetOpenIDConnectUserInfoResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_original_webhook_message

> models::MessageResponse get_original_webhook_message(webhook_id, webhook_token, thread_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** |  | [required] |
**webhook_token** | **String** |  | [required] |
**thread_id** | Option<**String**> |  |  |

### Return type

[**models::MessageResponse**](MessageResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_public_keys

> models::OAuth2GetKeys get_public_keys()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::OAuth2GetKeys**](OAuth2GetKeys.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_self_voice_state

> models::VoiceStateResponse get_self_voice_state(guild_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |

### Return type

[**models::VoiceStateResponse**](VoiceStateResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_soundboard_default_sounds

> Vec<models::SoundboardSoundResponse> get_soundboard_default_sounds()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::SoundboardSoundResponse>**](SoundboardSoundResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stage_instance

> models::StageInstanceResponse get_stage_instance(channel_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |

### Return type

[**models::StageInstanceResponse**](StageInstanceResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sticker

> models::GetSticker200Response get_sticker(sticker_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sticker_id** | **String** |  | [required] |

### Return type

[**models::GetSticker200Response**](get_sticker_200_response.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sticker_pack

> models::StickerPackResponse get_sticker_pack(pack_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pack_id** | **String** |  | [required] |

### Return type

[**models::StickerPackResponse**](StickerPackResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_thread_member

> models::ThreadMemberResponse get_thread_member(channel_id, user_id, with_member)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |
**with_member** | Option<**bool**> |  |  |

### Return type

[**models::ThreadMemberResponse**](ThreadMemberResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user

> models::UserResponse get_user(user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

[**models::UserResponse**](UserResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voice_state

> models::VoiceStateResponse get_voice_state(guild_id, user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

[**models::VoiceStateResponse**](VoiceStateResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webhook

> models::ListChannelWebhooks200ResponseInner get_webhook(webhook_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** |  | [required] |

### Return type

[**models::ListChannelWebhooks200ResponseInner**](list_channel_webhooks_200_response_inner.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webhook_by_token

> models::ListChannelWebhooks200ResponseInner get_webhook_by_token(webhook_id, webhook_token)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** |  | [required] |
**webhook_token** | **String** |  | [required] |

### Return type

[**models::ListChannelWebhooks200ResponseInner**](list_channel_webhooks_200_response_inner.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webhook_message

> models::MessageResponse get_webhook_message(webhook_id, webhook_token, message_id, thread_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** |  | [required] |
**webhook_token** | **String** |  | [required] |
**message_id** | **String** |  | [required] |
**thread_id** | Option<**String**> |  |  |

### Return type

[**models::MessageResponse**](MessageResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invite_resolve

> models::CreateChannelInvite200Response invite_resolve(code, with_counts, guild_scheduled_event_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** |  | [required] |
**with_counts** | Option<**bool**> |  |  |
**guild_scheduled_event_id** | Option<**String**> |  |  |

### Return type

[**models::CreateChannelInvite200Response**](create_channel_invite_200_response.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invite_revoke

> models::CreateChannelInvite200Response invite_revoke(code)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** |  | [required] |

### Return type

[**models::CreateChannelInvite200Response**](create_channel_invite_200_response.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## join_thread

> join_thread(channel_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## leave_guild

> leave_guild(guild_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## leave_lobby

> leave_lobby(lobby_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lobby_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## leave_thread

> leave_thread(channel_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_application_commands

> Vec<models::ApplicationCommandResponse> list_application_commands(application_id, with_localizations)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** |  | [required] |
**with_localizations** | Option<**bool**> |  |  |

### Return type

[**Vec<models::ApplicationCommandResponse>**](ApplicationCommandResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_application_emojis

> models::ListApplicationEmojisResponse list_application_emojis(application_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** |  | [required] |

### Return type

[**models::ListApplicationEmojisResponse**](ListApplicationEmojisResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_auto_moderation_rules

> Vec<models::ListAutoModerationRules200ResponseInner> list_auto_moderation_rules(guild_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |

### Return type

[**Vec<models::ListAutoModerationRules200ResponseInner>**](list_auto_moderation_rules_200_response_inner.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_channel_invites

> Vec<models::ListChannelInvites200ResponseInner> list_channel_invites(channel_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |

### Return type

[**Vec<models::ListChannelInvites200ResponseInner>**](list_channel_invites_200_response_inner.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_channel_webhooks

> Vec<models::ListChannelWebhooks200ResponseInner> list_channel_webhooks(channel_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |

### Return type

[**Vec<models::ListChannelWebhooks200ResponseInner>**](list_channel_webhooks_200_response_inner.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_guild_application_command_permissions

> Vec<models::CommandPermissionsResponse> list_guild_application_command_permissions(application_id, guild_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** |  | [required] |
**guild_id** | **String** |  | [required] |

### Return type

[**Vec<models::CommandPermissionsResponse>**](CommandPermissionsResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_guild_application_commands

> Vec<models::ApplicationCommandResponse> list_guild_application_commands(application_id, guild_id, with_localizations)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** |  | [required] |
**guild_id** | **String** |  | [required] |
**with_localizations** | Option<**bool**> |  |  |

### Return type

[**Vec<models::ApplicationCommandResponse>**](ApplicationCommandResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_guild_audit_log_entries

> models::GuildAuditLogResponse list_guild_audit_log_entries(guild_id, user_id, target_id, action_type, before, after, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**user_id** | Option<**String**> |  |  |
**target_id** | Option<**String**> |  |  |
**action_type** | Option<**i32**> |  |  |
**before** | Option<**String**> |  |  |
**after** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |

### Return type

[**models::GuildAuditLogResponse**](GuildAuditLogResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_guild_bans

> Vec<models::GuildBanResponse> list_guild_bans(guild_id, limit, before, after)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**limit** | Option<**i32**> |  |  |
**before** | Option<**String**> |  |  |
**after** | Option<**String**> |  |  |

### Return type

[**Vec<models::GuildBanResponse>**](GuildBanResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_guild_channels

> Vec<models::GetChannel200Response> list_guild_channels(guild_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |

### Return type

[**Vec<models::GetChannel200Response>**](get_channel_200_response.md)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_guild_emojis

> Vec<models::EmojiResponse> list_guild_emojis(guild_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |

### Return type

[**Vec<models::EmojiResponse>**](EmojiResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_guild_integrations

> Vec<models::ListGuildIntegrations200ResponseInner> list_guild_integrations(guild_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |

### Return type

[**Vec<models::ListGuildIntegrations200ResponseInner>**](list_guild_integrations_200_response_inner.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_guild_invites

> Vec<models::ListChannelInvites200ResponseInner> list_guild_invites(guild_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |

### Return type

[**Vec<models::ListChannelInvites200ResponseInner>**](list_channel_invites_200_response_inner.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_guild_members

> Vec<models::GuildMemberResponse> list_guild_members(guild_id, limit, after)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**limit** | Option<**i32**> |  |  |
**after** | Option<**i32**> |  |  |

### Return type

[**Vec<models::GuildMemberResponse>**](GuildMemberResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_guild_roles

> Vec<models::GuildRoleResponse> list_guild_roles(guild_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |

### Return type

[**Vec<models::GuildRoleResponse>**](GuildRoleResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_guild_scheduled_event_users

> Vec<models::ScheduledEventUserResponse> list_guild_scheduled_event_users(guild_id, guild_scheduled_event_id, with_member, limit, before, after)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**guild_scheduled_event_id** | **String** |  | [required] |
**with_member** | Option<**bool**> |  |  |
**limit** | Option<**i32**> |  |  |
**before** | Option<**String**> |  |  |
**after** | Option<**String**> |  |  |

### Return type

[**Vec<models::ScheduledEventUserResponse>**](ScheduledEventUserResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_guild_scheduled_events

> Vec<models::ListGuildScheduledEvents200ResponseInner> list_guild_scheduled_events(guild_id, with_user_count)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**with_user_count** | Option<**bool**> |  |  |

### Return type

[**Vec<models::ListGuildScheduledEvents200ResponseInner>**](list_guild_scheduled_events_200_response_inner.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_guild_soundboard_sounds

> models::ListGuildSoundboardSoundsResponse list_guild_soundboard_sounds(guild_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |

### Return type

[**models::ListGuildSoundboardSoundsResponse**](ListGuildSoundboardSoundsResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_guild_stickers

> Vec<models::GuildStickerResponse> list_guild_stickers(guild_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |

### Return type

[**Vec<models::GuildStickerResponse>**](GuildStickerResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_guild_templates

> Vec<models::GuildTemplateResponse> list_guild_templates(guild_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |

### Return type

[**Vec<models::GuildTemplateResponse>**](GuildTemplateResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_guild_voice_regions

> Vec<models::VoiceRegionResponse> list_guild_voice_regions(guild_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |

### Return type

[**Vec<models::VoiceRegionResponse>**](VoiceRegionResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_message_reactions_by_emoji

> Vec<models::UserResponse> list_message_reactions_by_emoji(channel_id, message_id, emoji_name, after, limit, r#type)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**message_id** | **String** |  | [required] |
**emoji_name** | **String** |  | [required] |
**after** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |
**r#type** | Option<**i32**> |  |  |

### Return type

[**Vec<models::UserResponse>**](UserResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_messages

> Vec<models::MessageResponse> list_messages(channel_id, around, before, after, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**around** | Option<**String**> |  |  |
**before** | Option<**String**> |  |  |
**after** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |

### Return type

[**Vec<models::MessageResponse>**](MessageResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_my_connections

> Vec<models::ConnectedAccountResponse> list_my_connections()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ConnectedAccountResponse>**](ConnectedAccountResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_my_guilds

> Vec<models::MyGuildResponse> list_my_guilds(before, after, limit, with_counts)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**before** | Option<**String**> |  |  |
**after** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |
**with_counts** | Option<**bool**> |  |  |

### Return type

[**Vec<models::MyGuildResponse>**](MyGuildResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_my_private_archived_threads

> models::ThreadsResponse list_my_private_archived_threads(channel_id, before, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**before** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |

### Return type

[**models::ThreadsResponse**](ThreadsResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_pins

> models::PinnedMessagesResponse list_pins(channel_id, before, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**before** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |

### Return type

[**models::PinnedMessagesResponse**](PinnedMessagesResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_private_archived_threads

> models::ThreadsResponse list_private_archived_threads(channel_id, before, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**before** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |

### Return type

[**models::ThreadsResponse**](ThreadsResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_public_archived_threads

> models::ThreadsResponse list_public_archived_threads(channel_id, before, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**before** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |

### Return type

[**models::ThreadsResponse**](ThreadsResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_sticker_packs

> models::StickerPackCollectionResponse list_sticker_packs()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::StickerPackCollectionResponse**](StickerPackCollectionResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_thread_members

> Vec<models::ThreadMemberResponse> list_thread_members(channel_id, with_member, limit, after)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**with_member** | Option<**bool**> |  |  |
**limit** | Option<**i32**> |  |  |
**after** | Option<**String**> |  |  |

### Return type

[**Vec<models::ThreadMemberResponse>**](ThreadMemberResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_voice_regions

> Vec<models::VoiceRegionResponse> list_voice_regions()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::VoiceRegionResponse>**](VoiceRegionResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## partner_sdk_token

> models::ProvisionalTokenResponse partner_sdk_token(partner_sdk_unmerge_provisional_account_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**partner_sdk_unmerge_provisional_account_request** | [**PartnerSdkUnmergeProvisionalAccountRequest**](PartnerSdkUnmergeProvisionalAccountRequest.md) |  | [required] |

### Return type

[**models::ProvisionalTokenResponse**](ProvisionalTokenResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## partner_sdk_unmerge_provisional_account

> partner_sdk_unmerge_provisional_account(partner_sdk_unmerge_provisional_account_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**partner_sdk_unmerge_provisional_account_request** | [**PartnerSdkUnmergeProvisionalAccountRequest**](PartnerSdkUnmergeProvisionalAccountRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## poll_expire

> models::MessageResponse poll_expire(channel_id, message_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**message_id** | **String** |  | [required] |

### Return type

[**models::MessageResponse**](MessageResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## preview_prune_guild

> models::GuildPruneResponse preview_prune_guild(guild_id, days, include_roles)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**days** | Option<**i32**> |  |  |
**include_roles** | Option<**String**> |  |  |

### Return type

[**models::GuildPruneResponse**](GuildPruneResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prune_guild

> models::GuildPruneResponse prune_guild(guild_id, prune_guild_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**prune_guild_request** | [**PruneGuildRequest**](PruneGuildRequest.md) |  | [required] |

### Return type

[**models::GuildPruneResponse**](GuildPruneResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_guilds_onboarding

> models::GuildOnboardingResponse put_guilds_onboarding(guild_id, update_guild_onboarding_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**update_guild_onboarding_request** | [**UpdateGuildOnboardingRequest**](UpdateGuildOnboardingRequest.md) |  | [required] |

### Return type

[**models::GuildOnboardingResponse**](GuildOnboardingResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_guild_members

> Vec<models::GuildMemberResponse> search_guild_members(query, guild_id, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** |  | [required] |
**guild_id** | **String** |  | [required] |
**limit** | Option<**i32**> |  |  |

### Return type

[**Vec<models::GuildMemberResponse>**](GuildMemberResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_soundboard_sound

> send_soundboard_sound(channel_id, soundboard_sound_send_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**soundboard_sound_send_request** | [**SoundboardSoundSendRequest**](SoundboardSoundSendRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_channel_permission_overwrite

> set_channel_permission_overwrite(channel_id, overwrite_id, set_channel_permission_overwrite_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**overwrite_id** | **String** |  | [required] |
**set_channel_permission_overwrite_request** | [**SetChannelPermissionOverwriteRequest**](SetChannelPermissionOverwriteRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_guild_application_command_permissions

> models::CommandPermissionsResponse set_guild_application_command_permissions(application_id, guild_id, command_id, set_guild_application_command_permissions_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** |  | [required] |
**guild_id** | **String** |  | [required] |
**command_id** | **String** |  | [required] |
**set_guild_application_command_permissions_request** | [**SetGuildApplicationCommandPermissionsRequest**](SetGuildApplicationCommandPermissionsRequest.md) |  | [required] |

### Return type

[**models::CommandPermissionsResponse**](CommandPermissionsResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_guild_template

> models::GuildTemplateResponse sync_guild_template(guild_id, code)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**code** | **String** |  | [required] |

### Return type

[**models::GuildTemplateResponse**](GuildTemplateResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## thread_search

> models::ThreadSearchResponse thread_search(channel_id, name, slop, min_id, max_id, tag, tag_setting, archived, sort_by, sort_order, limit, offset)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**name** | Option<**String**> |  |  |
**slop** | Option<**i32**> |  |  |
**min_id** | Option<**String**> |  |  |
**max_id** | Option<**String**> |  |  |
**tag** | Option<**String**> |  |  |
**tag_setting** | Option<**String**> |  |  |
**archived** | Option<**bool**> |  |  |
**sort_by** | Option<**String**> |  |  |
**sort_order** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |
**offset** | Option<**i32**> |  |  |

### Return type

[**models::ThreadSearchResponse**](ThreadSearchResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_typing_indicator

> serde_json::Value trigger_typing_indicator(channel_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unban_user_from_guild

> unban_user_from_guild(guild_id, user_id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_application

> models::PrivateApplicationResponse update_application(application_id, application_form_partial)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** |  | [required] |
**application_form_partial** | [**ApplicationFormPartial**](ApplicationFormPartial.md) |  | [required] |

### Return type

[**models::PrivateApplicationResponse**](PrivateApplicationResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_application_command

> models::ApplicationCommandResponse update_application_command(application_id, command_id, application_command_patch_request_partial)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** |  | [required] |
**command_id** | **String** |  | [required] |
**application_command_patch_request_partial** | [**ApplicationCommandPatchRequestPartial**](ApplicationCommandPatchRequestPartial.md) |  | [required] |

### Return type

[**models::ApplicationCommandResponse**](ApplicationCommandResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_application_emoji

> models::EmojiResponse update_application_emoji(application_id, emoji_id, update_application_emoji_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** |  | [required] |
**emoji_id** | **String** |  | [required] |
**update_application_emoji_request** | [**UpdateApplicationEmojiRequest**](UpdateApplicationEmojiRequest.md) |  | [required] |

### Return type

[**models::EmojiResponse**](EmojiResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_application_role_connections_metadata

> Vec<models::ApplicationRoleConnectionsMetadataItemResponse> update_application_role_connections_metadata(application_id, application_role_connections_metadata_item_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** |  | [required] |
**application_role_connections_metadata_item_request** | Option<[**Vec<models::ApplicationRoleConnectionsMetadataItemRequest>**](ApplicationRoleConnectionsMetadataItemRequest.md)> |  | [required] |

### Return type

[**Vec<models::ApplicationRoleConnectionsMetadataItemResponse>**](ApplicationRoleConnectionsMetadataItemResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_application_user_role_connection

> models::ApplicationUserRoleConnectionResponse update_application_user_role_connection(application_id, update_application_user_role_connection_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** |  | [required] |
**update_application_user_role_connection_request** | [**UpdateApplicationUserRoleConnectionRequest**](UpdateApplicationUserRoleConnectionRequest.md) |  | [required] |

### Return type

[**models::ApplicationUserRoleConnectionResponse**](ApplicationUserRoleConnectionResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_auto_moderation_rule

> models::CreateAutoModerationRule200Response update_auto_moderation_rule(guild_id, rule_id, update_auto_moderation_rule_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**rule_id** | **String** |  | [required] |
**update_auto_moderation_rule_request** | [**UpdateAutoModerationRuleRequest**](UpdateAutoModerationRuleRequest.md) |  | [required] |

### Return type

[**models::CreateAutoModerationRule200Response**](create_auto_moderation_rule_200_response.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_channel

> models::GetChannel200Response update_channel(channel_id, update_channel_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**update_channel_request** | [**UpdateChannelRequest**](UpdateChannelRequest.md) |  | [required] |

### Return type

[**models::GetChannel200Response**](get_channel_200_response.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_guild

> models::GuildResponse update_guild(guild_id, guild_patch_request_partial)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**guild_patch_request_partial** | [**GuildPatchRequestPartial**](GuildPatchRequestPartial.md) |  | [required] |

### Return type

[**models::GuildResponse**](GuildResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_guild_application_command

> models::ApplicationCommandResponse update_guild_application_command(application_id, guild_id, command_id, application_command_patch_request_partial)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** |  | [required] |
**guild_id** | **String** |  | [required] |
**command_id** | **String** |  | [required] |
**application_command_patch_request_partial** | [**ApplicationCommandPatchRequestPartial**](ApplicationCommandPatchRequestPartial.md) |  | [required] |

### Return type

[**models::ApplicationCommandResponse**](ApplicationCommandResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_guild_emoji

> models::EmojiResponse update_guild_emoji(guild_id, emoji_id, update_guild_emoji_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**emoji_id** | **String** |  | [required] |
**update_guild_emoji_request** | [**UpdateGuildEmojiRequest**](UpdateGuildEmojiRequest.md) |  | [required] |

### Return type

[**models::EmojiResponse**](EmojiResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_guild_member

> models::GuildMemberResponse update_guild_member(guild_id, user_id, update_guild_member_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |
**update_guild_member_request** | [**UpdateGuildMemberRequest**](UpdateGuildMemberRequest.md) |  | [required] |

### Return type

[**models::GuildMemberResponse**](GuildMemberResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_guild_role

> models::GuildRoleResponse update_guild_role(guild_id, role_id, update_role_request_partial)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**role_id** | **String** |  | [required] |
**update_role_request_partial** | [**UpdateRoleRequestPartial**](UpdateRoleRequestPartial.md) |  | [required] |

### Return type

[**models::GuildRoleResponse**](GuildRoleResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_guild_scheduled_event

> models::ListGuildScheduledEvents200ResponseInner update_guild_scheduled_event(guild_id, guild_scheduled_event_id, update_guild_scheduled_event_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**guild_scheduled_event_id** | **String** |  | [required] |
**update_guild_scheduled_event_request** | [**UpdateGuildScheduledEventRequest**](UpdateGuildScheduledEventRequest.md) |  | [required] |

### Return type

[**models::ListGuildScheduledEvents200ResponseInner**](list_guild_scheduled_events_200_response_inner.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_guild_soundboard_sound

> models::SoundboardSoundResponse update_guild_soundboard_sound(guild_id, sound_id, soundboard_patch_request_partial)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**sound_id** | **String** |  | [required] |
**soundboard_patch_request_partial** | [**SoundboardPatchRequestPartial**](SoundboardPatchRequestPartial.md) |  | [required] |

### Return type

[**models::SoundboardSoundResponse**](SoundboardSoundResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_guild_sticker

> models::GuildStickerResponse update_guild_sticker(guild_id, sticker_id, update_guild_sticker_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**sticker_id** | **String** |  | [required] |
**update_guild_sticker_request** | [**UpdateGuildStickerRequest**](UpdateGuildStickerRequest.md) |  | [required] |

### Return type

[**models::GuildStickerResponse**](GuildStickerResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_guild_template

> models::GuildTemplateResponse update_guild_template(guild_id, code, update_guild_template_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**code** | **String** |  | [required] |
**update_guild_template_request** | [**UpdateGuildTemplateRequest**](UpdateGuildTemplateRequest.md) |  | [required] |

### Return type

[**models::GuildTemplateResponse**](GuildTemplateResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_guild_welcome_screen

> models::GuildWelcomeScreenResponse update_guild_welcome_screen(guild_id, welcome_screen_patch_request_partial)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**welcome_screen_patch_request_partial** | [**WelcomeScreenPatchRequestPartial**](WelcomeScreenPatchRequestPartial.md) |  | [required] |

### Return type

[**models::GuildWelcomeScreenResponse**](GuildWelcomeScreenResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_guild_widget_settings

> models::WidgetSettingsResponse update_guild_widget_settings(guild_id, update_guild_widget_settings_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**update_guild_widget_settings_request** | [**UpdateGuildWidgetSettingsRequest**](UpdateGuildWidgetSettingsRequest.md) |  | [required] |

### Return type

[**models::WidgetSettingsResponse**](WidgetSettingsResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_message

> models::MessageResponse update_message(channel_id, message_id, message_edit_request_partial)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**message_id** | **String** |  | [required] |
**message_edit_request_partial** | [**MessageEditRequestPartial**](MessageEditRequestPartial.md) |  | [required] |

### Return type

[**models::MessageResponse**](MessageResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_my_application

> models::PrivateApplicationResponse update_my_application(application_form_partial)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_form_partial** | [**ApplicationFormPartial**](ApplicationFormPartial.md) |  | [required] |

### Return type

[**models::PrivateApplicationResponse**](PrivateApplicationResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_my_guild_member

> models::PrivateGuildMemberResponse update_my_guild_member(guild_id, update_my_guild_member_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**update_my_guild_member_request** | [**UpdateMyGuildMemberRequest**](UpdateMyGuildMemberRequest.md) |  | [required] |

### Return type

[**models::PrivateGuildMemberResponse**](PrivateGuildMemberResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_my_user

> models::UserPiiResponse update_my_user(bot_account_patch_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_account_patch_request** | [**BotAccountPatchRequest**](BotAccountPatchRequest.md) |  | [required] |

### Return type

[**models::UserPiiResponse**](UserPIIResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_original_webhook_message

> models::MessageResponse update_original_webhook_message(webhook_id, webhook_token, incoming_webhook_update_request_partial, thread_id, with_components)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** |  | [required] |
**webhook_token** | **String** |  | [required] |
**incoming_webhook_update_request_partial** | [**IncomingWebhookUpdateRequestPartial**](IncomingWebhookUpdateRequestPartial.md) |  | [required] |
**thread_id** | Option<**String**> |  |  |
**with_components** | Option<**bool**> |  |  |

### Return type

[**models::MessageResponse**](MessageResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_self_voice_state

> update_self_voice_state(guild_id, update_self_voice_state_request_partial)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**update_self_voice_state_request_partial** | [**UpdateSelfVoiceStateRequestPartial**](UpdateSelfVoiceStateRequestPartial.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_stage_instance

> models::StageInstanceResponse update_stage_instance(channel_id, update_stage_instance_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** |  | [required] |
**update_stage_instance_request** | [**UpdateStageInstanceRequest**](UpdateStageInstanceRequest.md) |  | [required] |

### Return type

[**models::StageInstanceResponse**](StageInstanceResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_voice_state

> update_voice_state(guild_id, user_id, update_voice_state_request_partial)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |
**update_voice_state_request_partial** | [**UpdateVoiceStateRequestPartial**](UpdateVoiceStateRequestPartial.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_webhook

> models::ListChannelWebhooks200ResponseInner update_webhook(webhook_id, update_webhook_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** |  | [required] |
**update_webhook_request** | [**UpdateWebhookRequest**](UpdateWebhookRequest.md) |  | [required] |

### Return type

[**models::ListChannelWebhooks200ResponseInner**](list_channel_webhooks_200_response_inner.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_webhook_by_token

> models::ListChannelWebhooks200ResponseInner update_webhook_by_token(webhook_id, webhook_token, update_webhook_by_token_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** |  | [required] |
**webhook_token** | **String** |  | [required] |
**update_webhook_by_token_request** | [**UpdateWebhookByTokenRequest**](UpdateWebhookByTokenRequest.md) |  | [required] |

### Return type

[**models::ListChannelWebhooks200ResponseInner**](list_channel_webhooks_200_response_inner.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_webhook_message

> models::MessageResponse update_webhook_message(webhook_id, webhook_token, message_id, incoming_webhook_update_request_partial, thread_id, with_components)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** |  | [required] |
**webhook_token** | **String** |  | [required] |
**message_id** | **String** |  | [required] |
**incoming_webhook_update_request_partial** | [**IncomingWebhookUpdateRequestPartial**](IncomingWebhookUpdateRequestPartial.md) |  | [required] |
**thread_id** | Option<**String**> |  |  |
**with_components** | Option<**bool**> |  |  |

### Return type

[**models::MessageResponse**](MessageResponse.md)

### Authorization

[BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_application_attachment

> models::ActivitiesAttachmentResponse upload_application_attachment(application_id, file)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** |  | [required] |
**file** | **String** |  | [required] |

### Return type

[**models::ActivitiesAttachmentResponse**](ActivitiesAttachmentResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [OAuth2](../README.md#OAuth2), [BotToken](../README.md#BotToken)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

