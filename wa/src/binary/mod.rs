use core::{fmt, fmt::Display};
mod decoder;
pub(crate) mod node;

/// ### Chat identification
/// The WhatsApp Web API uses the following formats to identify chats with
/// individual users and groups of multiple users.
/// - **Chats**: `[country code][number]@c.us`, e.g. **`49123456789@c.us`** when
///   you are from
/// Germany and your phone number is `0123 456789`.
/// - **Groups**: `[phone number of group creator]-[timestamp of group
///   creation]@g.us`, e.g.
/// **`49123456789-1509911919@g.us`** for the group that `49123456789@c.us`
/// created on November 5 2017.
/// - **Broadcast Channels** `[timestamp of broadcast channel
///   creation]@broadcast`, e.g.
/// **`1509911919@broadcast`** for an broadcast channel created on November 5
/// 2017.
#[derive(Debug, Clone, Default, Hash)]
pub struct Jid {
    id: String,
    is_group: bool,
    is_broadcast: bool,
    is_remote: bool,
}

impl Display for Jid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let suffix = if self.is_group {
            "@g.us"
        } else if self.is_broadcast {
            "@broadcast"
        } else if self.is_remote {
            "@s.whatsapp.net"
        } else {
            "@c.us"
        };

        write!(f, "{}{}", self.id, suffix)
    }
}
