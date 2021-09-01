use crate::types::{Message, Channel, Member, User};

#[derive(Serialize, Deserialize)]
#[serde(tag = "c", content = "d")]
pub enum WsInboundEvent {
    Identify {
        token: String,
        intents: u64,
    },
    Resume {
        token: String,
        session_id: String,
        sequence: u64,
    },
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "c", content = "d")]
/// Server -> client WebSocket events.
pub enum WsOutboundEvent {
    /// Fired when a new message is created.
    ///
    /// `message` is the message that was created.
    MessageCreate { message: Message },
    /// Fired when a message is updated.
    ///
    /// `old` is the message before the edit.
    /// `new` is the message after the edit.
    MessageUpdate { old: Message, new: Message },
    /// Fired when a message is deleted.
    ///
    /// `message` is the message that was deleted.
    MessageDelete { message: Message },

    ChannelCreate { channel: Channel },
    ChannelUpdate { old: Channel, new: Channel },
    ChannelDelete { channel: Channel },

    MemberCreate { member: Member },
    MemberUpdate { old: Member, new: Member },
    MemberDelete { member: Member },

    UserCreate { user: User }
}
