use crate::types::{Channel, Guild, Invite, Member, Message, Role, User};
use std::boxed::Box;

#[derive(Serialize, Deserialize)]
#[serde(tag = "c", content = "d")]
pub enum WsInboundEvent {
    Identify { token: String, intents: u64 },
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "c", content = "d")]
/// Server -> client WebSocket events.
pub enum WsOutboundEvent {
    /// Fired when an `Identify` is accepted as valid.
    ///
    /// `user` is the now-authorized user.
    IdentifyAccepted {
        user: User,
    },

    /// Fired when a new message is created.
    ///
    /// `message` is the message that was created.
    MessageCreate {
        message: Message,
    },
    /// Fired when a message is updated.
    ///
    /// `old` is the message before the edit.
    /// `new` is the message after the edit.
    MessageUpdate {
        old: Message,
        new: Message,
    },
    /// Fired when a message is deleted.
    ///
    /// `message` is the message that was deleted.
    MessageDelete {
        message: Message,
    },

    ChannelCreate {
        channel: Channel,
    },
    ChannelUpdate {
        old: Channel,
        new: Channel,
    },
    ChannelDelete {
        channel: Channel,
    },

    GuildCreate {
        guild: Guild,
    },
    GuildUpdate {
        old: Guild,
        new: Guild,
    },
    GuildDelete {
        guild: Guild,
    },

    MemberCreate {
        member: Member,
    },
    MemberUpdate {
        old: Box<Member>,
        new: Box<Member>,
    },
    MemberDelete {
        member: Member,
    },

    InviteCreate {
        invite: Invite,
    },
    InviteDelete {
        invite: Invite,
    },

    RoleCreate {
        role: Role,
    },
    RoleUpdate {
        old: Role,
        new: Role,
    },
    RoleDelete {
        role: Role,
    },

    MemberRoleAdd {
        member: Member,
        role: Role,
    },
    MemberRoleDelete {
        member: Member,
        role: Role,
    },
}
