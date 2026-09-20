use serenity::all::{
    ChannelType, CreateChannel, GuildId, PermissionOverwrite, PermissionOverwriteType,
    Permissions, RoleId, UserId,
};
use serenity::client::Context;

pub async fn create_ticket_channel(
    ctx: &Context,
    guild_id: GuildId,
    category_channel_id: u64,
    support_role_id: u64,
    user_id: UserId,
    username: &str,
    category_key: &str,
) -> anyhow::Result<serenity::model::channel::GuildChannel> {
    // 1. Права: запрещаем просмотр всем (@everyone = ID гильдии)
    let everyone_overwrite = PermissionOverwrite {
        allow: Permissions::empty(),
        deny: Permissions::VIEW_CHANNEL,
        kind: PermissionOverwriteType::Role(RoleId::new(guild_id.get())),
    };

    // 2. Права: разрешаем просмотр и отправку сообщений автору тикета
    let user_overwrite = PermissionOverwrite {
        allow: Permissions::VIEW_CHANNEL
            | Permissions::SEND_MESSAGES
            | Permissions::ATTACH_FILES
            | Permissions::READ_MESSAGE_HISTORY,
        deny: Permissions::empty(),
        kind: PermissionOverwriteType::Member(user_id),
    };

    // 3. Права: разрешаем просмотр и управление саппортам
    let support_overwrite = PermissionOverwrite {
        allow: Permissions::VIEW_CHANNEL
            | Permissions::SEND_MESSAGES
            | Permissions::ATTACH_FILES
            | Permissions::READ_MESSAGE_HISTORY
            | Permissions::MANAGE_MESSAGES,
        deny: Permissions::empty(),
        kind: PermissionOverwriteType::Role(RoleId::new(support_role_id)),
    };

    // Создание текстового канала
    let channel_name = format!("ticket-{}-{}", category_key, username);
    let builder = CreateChannel::new(channel_name)
        .kind(ChannelType::Text)
        .category(category_channel_id)
        .permissions(vec![everyone_overwrite, user_overwrite, support_overwrite]);

    let channel = guild_id.create_channel(&ctx.http, builder).await?;
    Ok(channel)
}