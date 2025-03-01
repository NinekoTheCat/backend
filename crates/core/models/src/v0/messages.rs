use crate::v0::MessageWebhook;
use indexmap::{IndexMap, IndexSet};
use iso8601_timestamp::Timestamp;
#[cfg(feature = "rocket_impl")]
use rocket::FromFormField;
use serde::{Deserialize, Serialize};

use super::{File, User};

/// Utility function to check if a boolean value is false
pub fn if_false(t: &bool) -> bool {
    !t
}

auto_derived!(
    /// # Reply
    ///
    /// Representation of a message reply before it is sent.
    pub struct Reply {
        /// Message Id
        pub id: String,
        /// Whether this reply should mention the message's author
        pub mention: bool,
    }

    /// Representation of a text embed before it is sent.
    pub struct SendableEmbed {
        #[validate(length(min = 1, max = 128))]
        pub icon_url: Option<String>,
        #[validate(length(min = 1, max = 256))]
        pub url: Option<String>,
        #[validate(length(min = 1, max = 100))]
        pub title: Option<String>,
        #[validate(length(min = 1, max = 2000))]
        pub description: Option<String>,
        pub media: Option<String>,
        #[validate(length(min = 1, max = 128))]
        pub colour: Option<String>,
    }

    /// Representation of a system event message
    #[serde(tag = "type")]
    pub enum SystemMessage {
        #[serde(rename = "text")]
        Text { content: String },
        #[serde(rename = "user_added")]
        UserAdded { id: String, by: String },
        #[serde(rename = "user_remove")]
        UserRemove { id: String, by: String },
        #[serde(rename = "user_joined")]
        UserJoined { id: String },
        #[serde(rename = "user_left")]
        UserLeft { id: String },
        #[serde(rename = "user_kicked")]
        UserKicked { id: String },
        #[serde(rename = "user_banned")]
        UserBanned { id: String },
        #[serde(rename = "channel_renamed")]
        ChannelRenamed { name: String, by: String },
        #[serde(rename = "channel_description_changed")]
        ChannelDescriptionChanged { by: String },
        #[serde(rename = "channel_icon_changed")]
        ChannelIconChanged { by: String },
        #[serde(rename = "channel_ownership_changed")]
        ChannelOwnershipChanged { from: String, to: String },
    }

    /// Name and / or avatar override information
    pub struct Masquerade {
        /// Replace the display name shown on this message
        #[serde(skip_serializing_if = "Option::is_none")]
        #[validate(length(min = 1, max = 32))]
        pub name: Option<String>,
        /// Replace the avatar shown on this message (URL to image file)
        #[serde(skip_serializing_if = "Option::is_none")]
        #[validate(length(min = 1, max = 256))]
        pub avatar: Option<String>,
        /// Replace the display role colour shown on this message
        ///
        /// Must have `ManageRole` permission to use
        #[serde(skip_serializing_if = "Option::is_none")]
        #[validate(length(min = 1, max = 128))]
        pub colour: Option<String>,
    }

    /// Information to guide interactions on this message
    pub struct Interactions {
        /// Reactions which should always appear and be distinct
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub reactions: Option<IndexSet<String>>,
        /// Whether reactions should be restricted to the given list
        ///
        /// Can only be set to true if reactions list is of at least length 1
        #[serde(skip_serializing_if = "if_false", default)]
        pub restrict_reactions: bool,
    }

    /// # Message Sort
    ///
    /// Sort used for retrieving messages
    #[derive(Default)]
    pub enum MessageSort {
        /// Sort by the most relevant messages
        #[default]
        Relevance,
        /// Sort by the newest messages first
        Latest,
        /// Sort by the oldest messages first
        Oldest,
    }

    /// # Message Time Period
    ///
    /// Filter and sort messages by time
    #[serde(untagged)]
    pub enum MessageTimePeriod {
        Relative {
            /// Message id to search around
            ///
            /// Specifying 'nearby' ignores 'before', 'after' and 'sort'.
            /// It will also take half of limit rounded as the limits to each side.
            /// It also fetches the message ID specified.
            nearby: String,
        },
        Absolute {
            /// Message id before which messages should be fetched
            before: Option<String>,
            /// Message id after which messages should be fetched
            after: Option<String>,
            /// Message sort direction
            sort: Option<MessageSort>,
        },
    }

    /// # Message Filter
    pub struct MessageFilter {
        /// Parent channel ID
        pub channel: Option<String>,
        /// Message author ID
        pub author: Option<String>,
        /// Search query
        pub query: Option<String>,
    }

    /// # Message Query
    pub struct MessageQuery {
        /// Maximum number of messages to fetch
        ///
        /// For fetching nearby messages, this is \`(limit + 1)\`.
        pub limit: Option<i64>,
        /// Filter to apply
        #[serde(flatten)]
        pub filter: MessageFilter,
        /// Time period to fetch
        #[serde(flatten)]
        pub time_period: MessageTimePeriod,
    }

    /// # Bulk Message Response
    ///
    /// Response used when multiple messages are fetched
    #[serde(untagged)]
    pub enum BulkMessageResponse {
        JustMessages(
            /// List of messages
            Vec<Message>,
        ),
        MessagesAndUsers {
            /// List of messages
            messages: Vec<Message>,
            /// List of users
            users: Vec<User>,
            /// List of members
            #[serde(skip_serializing_if = "Option::is_none")]
            members: Option<Vec<Member>>,
        },
    }

    /// # Appended Information
    pub struct AppendMessage {
        /// Additional embeds to include in this message
        #[serde(skip_serializing_if = "Option::is_none")]
        pub embeds: Option<Vec<Embed>>,
    }

    pub struct DataMessageSend {
        /// Unique token to prevent duplicate message sending
        ///
        /// **This is deprecated and replaced by `Idempotency-Key`!**
        #[validate(length(min = 1, max = 64))]
        pub nonce: Option<String>,

        /// Message content to send
        #[validate(length(min = 0, max = 2000))]
        pub content: Option<String>,
        /// Attachments to include in message
        pub attachments: Option<Vec<String>>,
        /// Messages to reply to
        pub replies: Option<Vec<Reply>>,
        /// Embeds to include in message
        ///
        /// Text embed content contributes to the content length cap
        #[validate]
        pub embeds: Option<Vec<SendableEmbed>>,
        /// Masquerade to apply to this message
        #[validate]
        pub masquerade: Option<Masquerade>,
        /// Information about how this message should be interacted with
        pub interactions: Option<Interactions>,
    }
);
auto_derived_partial!(
    /// Representation of a Message on Revolt
    #[opt_some_priority]
    pub struct Message {
        /// Unique Id
        #[serde(rename = "_id")]
        pub id: String,
        /// Unique value generated by client sending this message
        #[serde(skip_serializing_if = "Option::is_none")]
        pub nonce: Option<String>,
        /// Id of the channel this message was sent in
        pub channel: String,
        /// Id of the user or webhook that sent this message
        pub author: String,
        /// The webhook that sent this message
        #[serde(skip_serializing_if = "Option::is_none")]
        pub webhook: Option<MessageWebhook>,
        /// Message content
        #[serde(skip_serializing_if = "Option::is_none")]
        pub content: Option<String>,
        /// System message
        #[serde(skip_serializing_if = "Option::is_none")]
        pub system: Option<SystemMessage>,
        /// Array of attachments
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attachments: Option<Vec<File>>,
        /// Time at which this message was last edited
        #[serde(skip_serializing_if = "Option::is_none")]
        pub edited: Option<Timestamp>,
        /// Attached embeds to this message
        #[serde(skip_serializing_if = "Option::is_none")]
        pub embeds: Option<Vec<Embed>>,
        /// Array of user ids mentioned in this message
        #[serde(skip_serializing_if = "Option::is_none")]
        pub mentions: Option<Vec<String>>,
        /// Array of message ids this message is replying to
        #[serde(skip_serializing_if = "Option::is_none")]
        pub replies: Option<Vec<String>>,
        /// Hashmap of emoji IDs to array of user IDs
        #[serde(skip_serializing_if = "IndexMap::is_empty", default)]
        pub reactions: IndexMap<String, IndexSet<String>>,
        /// Information about how this message should be interacted with
        #[serde(skip_serializing_if = "Interactions::is_default", default)]
        pub interactions: Interactions,
        /// Name and / or avatar overrides for this message
        #[serde(skip_serializing_if = "Option::is_none")]
        pub masquerade: Option<Masquerade>,
    },
    "PartialMessage"
);
