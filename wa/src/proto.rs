#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HydratedQuickReplyButton {
    #[prost(string, optional, tag="1")]
    pub display_text: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="2")]
    pub id: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HydratedUrlButton {
    #[prost(string, optional, tag="1")]
    pub display_text: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="2")]
    pub url: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HydratedCallButton {
    #[prost(string, optional, tag="1")]
    pub display_text: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="2")]
    pub phone_number: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HydratedTemplateButton {
    #[prost(uint32, optional, tag="4")]
    pub index: ::std::option::Option<u32>,
    #[prost(oneof="hydrated_template_button::HydratedButton", tags="1, 2, 3")]
    pub hydrated_button: ::std::option::Option<hydrated_template_button::HydratedButton>,
}
pub mod hydrated_template_button {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum HydratedButton {
        #[prost(message, tag="1")]
        QuickReplyButton(super::HydratedQuickReplyButton),
        #[prost(message, tag="2")]
        UrlButton(super::HydratedUrlButton),
        #[prost(message, tag="3")]
        CallButton(super::HydratedCallButton),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuickReplyButton {
    #[prost(message, optional, tag="1")]
    pub display_text: ::std::option::Option<HighlyStructuredMessage>,
    #[prost(string, optional, tag="2")]
    pub id: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UrlButton {
    #[prost(message, optional, tag="1")]
    pub display_text: ::std::option::Option<HighlyStructuredMessage>,
    #[prost(message, optional, tag="2")]
    pub url: ::std::option::Option<HighlyStructuredMessage>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallButton {
    #[prost(message, optional, tag="1")]
    pub display_text: ::std::option::Option<HighlyStructuredMessage>,
    #[prost(message, optional, tag="2")]
    pub phone_number: ::std::option::Option<HighlyStructuredMessage>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TemplateButton {
    #[prost(uint32, optional, tag="4")]
    pub index: ::std::option::Option<u32>,
    #[prost(oneof="template_button::Button", tags="1, 2, 3")]
    pub button: ::std::option::Option<template_button::Button>,
}
pub mod template_button {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Button {
        #[prost(message, tag="1")]
        QuickReplyButton(super::QuickReplyButton),
        #[prost(message, tag="2")]
        UrlButton(super::UrlButton),
        #[prost(message, tag="3")]
        CallButton(super::CallButton),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Location {
    #[prost(double, optional, tag="1")]
    pub degrees_latitude: ::std::option::Option<f64>,
    #[prost(double, optional, tag="2")]
    pub degrees_longitude: ::std::option::Option<f64>,
    #[prost(string, optional, tag="3")]
    pub name: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Point {
    #[prost(int32, optional, tag="1")]
    pub x_deprecated: ::std::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub y_deprecated: ::std::option::Option<i32>,
    #[prost(double, optional, tag="3")]
    pub x: ::std::option::Option<f64>,
    #[prost(double, optional, tag="4")]
    pub y: ::std::option::Option<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InteractiveAnnotation {
    #[prost(message, repeated, tag="1")]
    pub polygon_vertices: ::std::vec::Vec<Point>,
    #[prost(oneof="interactive_annotation::Action", tags="2")]
    pub action: ::std::option::Option<interactive_annotation::Action>,
}
pub mod interactive_annotation {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Action {
        #[prost(message, tag="2")]
        Location(super::Location),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdReplyInfo {
    #[prost(string, optional, tag="1")]
    pub advertiser_name: ::std::option::Option<std::string::String>,
    #[prost(enumeration="ad_reply_info::AdReplyInfoMediatype", optional, tag="2")]
    pub media_type: ::std::option::Option<i32>,
    #[prost(bytes, optional, tag="16")]
    pub jpeg_thumbnail: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(string, optional, tag="17")]
    pub caption: ::std::option::Option<std::string::String>,
}
pub mod ad_reply_info {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AdReplyInfoMediatype {
        None = 0,
        Image = 1,
        Video = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContextInfo {
    #[prost(string, optional, tag="1")]
    pub stanza_id: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="2")]
    pub participant: ::std::option::Option<std::string::String>,
    #[prost(message, optional, boxed, tag="3")]
    pub quoted_message: ::std::option::Option<::std::boxed::Box<Message>>,
    #[prost(string, optional, tag="4")]
    pub remote_jid: ::std::option::Option<std::string::String>,
    #[prost(string, repeated, tag="15")]
    pub mentioned_jid: ::std::vec::Vec<std::string::String>,
    #[prost(string, optional, tag="18")]
    pub conversion_source: ::std::option::Option<std::string::String>,
    #[prost(bytes, optional, tag="19")]
    pub conversion_data: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="20")]
    pub conversion_delay_seconds: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="21")]
    pub forwarding_score: ::std::option::Option<u32>,
    #[prost(bool, optional, tag="22")]
    pub is_forwarded: ::std::option::Option<bool>,
    #[prost(message, optional, tag="23")]
    pub quoted_ad: ::std::option::Option<AdReplyInfo>,
    #[prost(message, optional, tag="24")]
    pub placeholder_key: ::std::option::Option<MessageKey>,
    #[prost(uint32, optional, tag="25")]
    pub expiration: ::std::option::Option<u32>,
    #[prost(int64, optional, tag="26")]
    pub ephemeral_setting_timestamp: ::std::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SenderKeyDistributionMessage {
    #[prost(string, optional, tag="1")]
    pub group_id: ::std::option::Option<std::string::String>,
    #[prost(bytes, optional, tag="2")]
    pub axolotl_sender_key_distribution_message: ::std::option::Option<std::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageMessage {
    #[prost(string, optional, tag="1")]
    pub url: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="2")]
    pub mimetype: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="3")]
    pub caption: ::std::option::Option<std::string::String>,
    #[prost(bytes, optional, tag="4")]
    pub file_sha256: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(uint64, optional, tag="5")]
    pub file_length: ::std::option::Option<u64>,
    #[prost(uint32, optional, tag="6")]
    pub height: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="7")]
    pub width: ::std::option::Option<u32>,
    #[prost(bytes, optional, tag="8")]
    pub media_key: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(bytes, optional, tag="9")]
    pub file_enc_sha256: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(message, repeated, tag="10")]
    pub interactive_annotations: ::std::vec::Vec<InteractiveAnnotation>,
    #[prost(string, optional, tag="11")]
    pub direct_path: ::std::option::Option<std::string::String>,
    #[prost(int64, optional, tag="12")]
    pub media_key_timestamp: ::std::option::Option<i64>,
    #[prost(bytes, optional, tag="16")]
    pub jpeg_thumbnail: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(message, optional, boxed, tag="17")]
    pub context_info: ::std::option::Option<::std::boxed::Box<ContextInfo>>,
    #[prost(bytes, optional, tag="18")]
    pub first_scan_sidecar: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="19")]
    pub first_scan_length: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="20")]
    pub experiment_group_id: ::std::option::Option<u32>,
    #[prost(bytes, optional, tag="21")]
    pub scans_sidecar: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(uint32, repeated, packed="false", tag="22")]
    pub scan_lengths: ::std::vec::Vec<u32>,
    #[prost(bytes, optional, tag="23")]
    pub mid_quality_file_sha256: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(bytes, optional, tag="24")]
    pub mid_quality_file_enc_sha256: ::std::option::Option<std::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContactMessage {
    #[prost(string, optional, tag="1")]
    pub display_name: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="16")]
    pub vcard: ::std::option::Option<std::string::String>,
    #[prost(message, optional, boxed, tag="17")]
    pub context_info: ::std::option::Option<::std::boxed::Box<ContextInfo>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationMessage {
    #[prost(double, optional, tag="1")]
    pub degrees_latitude: ::std::option::Option<f64>,
    #[prost(double, optional, tag="2")]
    pub degrees_longitude: ::std::option::Option<f64>,
    #[prost(string, optional, tag="3")]
    pub name: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="4")]
    pub address: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="5")]
    pub url: ::std::option::Option<std::string::String>,
    #[prost(bool, optional, tag="6")]
    pub is_live: ::std::option::Option<bool>,
    #[prost(uint32, optional, tag="7")]
    pub accuracy_in_meters: ::std::option::Option<u32>,
    #[prost(float, optional, tag="8")]
    pub speed_in_mps: ::std::option::Option<f32>,
    #[prost(uint32, optional, tag="9")]
    pub degrees_clockwise_from_magnetic_north: ::std::option::Option<u32>,
    #[prost(string, optional, tag="11")]
    pub comment: ::std::option::Option<std::string::String>,
    #[prost(bytes, optional, tag="16")]
    pub jpeg_thumbnail: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(message, optional, boxed, tag="17")]
    pub context_info: ::std::option::Option<::std::boxed::Box<ContextInfo>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtendedTextMessage {
    #[prost(string, optional, tag="1")]
    pub text: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="2")]
    pub matched_text: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="4")]
    pub canonical_url: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="5")]
    pub description: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="6")]
    pub title: ::std::option::Option<std::string::String>,
    #[prost(fixed32, optional, tag="7")]
    pub text_argb: ::std::option::Option<u32>,
    #[prost(fixed32, optional, tag="8")]
    pub background_argb: ::std::option::Option<u32>,
    #[prost(enumeration="extended_text_message::ExtendedTextMessageFonttype", optional, tag="9")]
    pub font: ::std::option::Option<i32>,
    #[prost(enumeration="extended_text_message::ExtendedTextMessagePreviewtype", optional, tag="10")]
    pub preview_type: ::std::option::Option<i32>,
    #[prost(bytes, optional, tag="16")]
    pub jpeg_thumbnail: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(message, optional, boxed, tag="17")]
    pub context_info: ::std::option::Option<::std::boxed::Box<ContextInfo>>,
    #[prost(bool, optional, tag="18")]
    pub do_not_play_inline: ::std::option::Option<bool>,
}
pub mod extended_text_message {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ExtendedTextMessageFonttype {
        SansSerif = 0,
        Serif = 1,
        NoricanRegular = 2,
        BryndanWrite = 3,
        BebasneueRegular = 4,
        OswaldHeavy = 5,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ExtendedTextMessagePreviewtype {
        None = 0,
        Video = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentMessage {
    #[prost(string, optional, tag="1")]
    pub url: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="2")]
    pub mimetype: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="3")]
    pub title: ::std::option::Option<std::string::String>,
    #[prost(bytes, optional, tag="4")]
    pub file_sha256: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(uint64, optional, tag="5")]
    pub file_length: ::std::option::Option<u64>,
    #[prost(uint32, optional, tag="6")]
    pub page_count: ::std::option::Option<u32>,
    #[prost(bytes, optional, tag="7")]
    pub media_key: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(string, optional, tag="8")]
    pub file_name: ::std::option::Option<std::string::String>,
    #[prost(bytes, optional, tag="9")]
    pub file_enc_sha256: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(string, optional, tag="10")]
    pub direct_path: ::std::option::Option<std::string::String>,
    #[prost(int64, optional, tag="11")]
    pub media_key_timestamp: ::std::option::Option<i64>,
    #[prost(bytes, optional, tag="16")]
    pub jpeg_thumbnail: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(message, optional, boxed, tag="17")]
    pub context_info: ::std::option::Option<::std::boxed::Box<ContextInfo>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudioMessage {
    #[prost(string, optional, tag="1")]
    pub url: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="2")]
    pub mimetype: ::std::option::Option<std::string::String>,
    #[prost(bytes, optional, tag="3")]
    pub file_sha256: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(uint64, optional, tag="4")]
    pub file_length: ::std::option::Option<u64>,
    #[prost(uint32, optional, tag="5")]
    pub seconds: ::std::option::Option<u32>,
    #[prost(bool, optional, tag="6")]
    pub ptt: ::std::option::Option<bool>,
    #[prost(bytes, optional, tag="7")]
    pub media_key: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(bytes, optional, tag="8")]
    pub file_enc_sha256: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(string, optional, tag="9")]
    pub direct_path: ::std::option::Option<std::string::String>,
    #[prost(int64, optional, tag="10")]
    pub media_key_timestamp: ::std::option::Option<i64>,
    #[prost(message, optional, boxed, tag="17")]
    pub context_info: ::std::option::Option<::std::boxed::Box<ContextInfo>>,
    #[prost(bytes, optional, tag="18")]
    pub streaming_sidecar: ::std::option::Option<std::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoMessage {
    #[prost(string, optional, tag="1")]
    pub url: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="2")]
    pub mimetype: ::std::option::Option<std::string::String>,
    #[prost(bytes, optional, tag="3")]
    pub file_sha256: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(uint64, optional, tag="4")]
    pub file_length: ::std::option::Option<u64>,
    #[prost(uint32, optional, tag="5")]
    pub seconds: ::std::option::Option<u32>,
    #[prost(bytes, optional, tag="6")]
    pub media_key: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(string, optional, tag="7")]
    pub caption: ::std::option::Option<std::string::String>,
    #[prost(bool, optional, tag="8")]
    pub gif_playback: ::std::option::Option<bool>,
    #[prost(uint32, optional, tag="9")]
    pub height: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="10")]
    pub width: ::std::option::Option<u32>,
    #[prost(bytes, optional, tag="11")]
    pub file_enc_sha256: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(message, repeated, tag="12")]
    pub interactive_annotations: ::std::vec::Vec<InteractiveAnnotation>,
    #[prost(string, optional, tag="13")]
    pub direct_path: ::std::option::Option<std::string::String>,
    #[prost(int64, optional, tag="14")]
    pub media_key_timestamp: ::std::option::Option<i64>,
    #[prost(bytes, optional, tag="16")]
    pub jpeg_thumbnail: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(message, optional, boxed, tag="17")]
    pub context_info: ::std::option::Option<::std::boxed::Box<ContextInfo>>,
    #[prost(bytes, optional, tag="18")]
    pub streaming_sidecar: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(enumeration="video_message::VideoMessageAttribution", optional, tag="19")]
    pub gif_attribution: ::std::option::Option<i32>,
}
pub mod video_message {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum VideoMessageAttribution {
        None = 0,
        Giphy = 1,
        Tenor = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Call {
    #[prost(bytes, optional, tag="1")]
    pub call_key: ::std::option::Option<std::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Chat {
    #[prost(string, optional, tag="1")]
    pub display_name: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="2")]
    pub id: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtocolMessage {
    #[prost(message, optional, tag="1")]
    pub key: ::std::option::Option<MessageKey>,
    #[prost(enumeration="protocol_message::ProtocolMessageType", optional, tag="2")]
    pub r#type: ::std::option::Option<i32>,
    #[prost(uint32, optional, tag="4")]
    pub ephemeral_expiration: ::std::option::Option<u32>,
    #[prost(int64, optional, tag="5")]
    pub ephemeral_setting_timestamp: ::std::option::Option<i64>,
    #[prost(message, optional, tag="6")]
    pub history_sync_notification: ::std::option::Option<HistorySyncNotification>,
}
pub mod protocol_message {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ProtocolMessageType {
        Revoke = 0,
        EphemeralSetting = 3,
        EphemeralSyncResponse = 4,
        HistorySyncNotification = 5,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistorySyncNotification {
    #[prost(bytes, optional, tag="1")]
    pub file_sha256: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(uint64, optional, tag="2")]
    pub file_length: ::std::option::Option<u64>,
    #[prost(bytes, optional, tag="3")]
    pub media_key: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(bytes, optional, tag="4")]
    pub file_enc_sha256: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(string, optional, tag="5")]
    pub direct_path: ::std::option::Option<std::string::String>,
    #[prost(enumeration="history_sync_notification::HistorySyncNotificationHistorysynctype", optional, tag="6")]
    pub sync_type: ::std::option::Option<i32>,
    #[prost(uint32, optional, tag="7")]
    pub chunk_order: ::std::option::Option<u32>,
}
pub mod history_sync_notification {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum HistorySyncNotificationHistorysynctype {
        InitialBootstrap = 0,
        InitialStatusV3 = 1,
        Full = 2,
        Recent = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContactsArrayMessage {
    #[prost(string, optional, tag="1")]
    pub display_name: ::std::option::Option<std::string::String>,
    #[prost(message, repeated, tag="2")]
    pub contacts: ::std::vec::Vec<ContactMessage>,
    #[prost(message, optional, boxed, tag="17")]
    pub context_info: ::std::option::Option<::std::boxed::Box<ContextInfo>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HsmCurrency {
    #[prost(string, optional, tag="1")]
    pub currency_code: ::std::option::Option<std::string::String>,
    #[prost(int64, optional, tag="2")]
    pub amount1000: ::std::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HsmDateTimeComponent {
    #[prost(enumeration="hsm_date_time_component::HsmDateTimeComponentDayofweektype", optional, tag="1")]
    pub day_of_week: ::std::option::Option<i32>,
    #[prost(uint32, optional, tag="2")]
    pub year: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub month: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="4")]
    pub day_of_month: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="5")]
    pub hour: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="6")]
    pub minute: ::std::option::Option<u32>,
    #[prost(enumeration="hsm_date_time_component::HsmDateTimeComponentCalendartype", optional, tag="7")]
    pub calendar: ::std::option::Option<i32>,
}
pub mod hsm_date_time_component {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum HsmDateTimeComponentDayofweektype {
        Monday = 1,
        Tuesday = 2,
        Wednesday = 3,
        Thursday = 4,
        Friday = 5,
        Saturday = 6,
        Sunday = 7,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum HsmDateTimeComponentCalendartype {
        Gregorian = 1,
        SolarHijri = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HsmDateTimeUnixEpoch {
    #[prost(int64, optional, tag="1")]
    pub timestamp: ::std::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HsmDateTime {
    #[prost(oneof="hsm_date_time::DatetimeOneof", tags="1, 2")]
    pub datetime_oneof: ::std::option::Option<hsm_date_time::DatetimeOneof>,
}
pub mod hsm_date_time {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DatetimeOneof {
        #[prost(message, tag="1")]
        Component(super::HsmDateTimeComponent),
        #[prost(message, tag="2")]
        UnixEpoch(super::HsmDateTimeUnixEpoch),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HsmLocalizableParameter {
    #[prost(string, optional, tag="1")]
    pub default: ::std::option::Option<std::string::String>,
    #[prost(oneof="hsm_localizable_parameter::ParamOneof", tags="2, 3")]
    pub param_oneof: ::std::option::Option<hsm_localizable_parameter::ParamOneof>,
}
pub mod hsm_localizable_parameter {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ParamOneof {
        #[prost(message, tag="2")]
        Currency(super::HsmCurrency),
        #[prost(message, tag="3")]
        DateTime(super::HsmDateTime),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HighlyStructuredMessage {
    #[prost(string, optional, tag="1")]
    pub namespace: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="2")]
    pub element_name: ::std::option::Option<std::string::String>,
    #[prost(string, repeated, tag="3")]
    pub params: ::std::vec::Vec<std::string::String>,
    #[prost(string, optional, tag="4")]
    pub fallback_lg: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="5")]
    pub fallback_lc: ::std::option::Option<std::string::String>,
    #[prost(message, repeated, tag="6")]
    pub localizable_params: ::std::vec::Vec<HsmLocalizableParameter>,
    #[prost(string, optional, tag="7")]
    pub deterministic_lg: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="8")]
    pub deterministic_lc: ::std::option::Option<std::string::String>,
    #[prost(message, optional, boxed, tag="9")]
    pub hydrated_hsm: ::std::option::Option<::std::boxed::Box<TemplateMessage>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendPaymentMessage {
    #[prost(message, optional, boxed, tag="2")]
    pub note_message: ::std::option::Option<::std::boxed::Box<Message>>,
    #[prost(message, optional, tag="3")]
    pub request_message_key: ::std::option::Option<MessageKey>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestPaymentMessage {
    #[prost(message, optional, boxed, tag="4")]
    pub note_message: ::std::option::Option<::std::boxed::Box<Message>>,
    #[prost(string, optional, tag="1")]
    pub currency_code_iso4217: ::std::option::Option<std::string::String>,
    #[prost(uint64, optional, tag="2")]
    pub amount1000: ::std::option::Option<u64>,
    #[prost(string, optional, tag="3")]
    pub request_from: ::std::option::Option<std::string::String>,
    #[prost(int64, optional, tag="5")]
    pub expiry_timestamp: ::std::option::Option<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeclinePaymentRequestMessage {
    #[prost(message, optional, tag="1")]
    pub key: ::std::option::Option<MessageKey>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelPaymentRequestMessage {
    #[prost(message, optional, tag="1")]
    pub key: ::std::option::Option<MessageKey>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveLocationMessage {
    #[prost(double, optional, tag="1")]
    pub degrees_latitude: ::std::option::Option<f64>,
    #[prost(double, optional, tag="2")]
    pub degrees_longitude: ::std::option::Option<f64>,
    #[prost(uint32, optional, tag="3")]
    pub accuracy_in_meters: ::std::option::Option<u32>,
    #[prost(float, optional, tag="4")]
    pub speed_in_mps: ::std::option::Option<f32>,
    #[prost(uint32, optional, tag="5")]
    pub degrees_clockwise_from_magnetic_north: ::std::option::Option<u32>,
    #[prost(string, optional, tag="6")]
    pub caption: ::std::option::Option<std::string::String>,
    #[prost(int64, optional, tag="7")]
    pub sequence_number: ::std::option::Option<i64>,
    #[prost(uint32, optional, tag="8")]
    pub time_offset: ::std::option::Option<u32>,
    #[prost(bytes, optional, tag="16")]
    pub jpeg_thumbnail: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(message, optional, boxed, tag="17")]
    pub context_info: ::std::option::Option<::std::boxed::Box<ContextInfo>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StickerMessage {
    #[prost(string, optional, tag="1")]
    pub url: ::std::option::Option<std::string::String>,
    #[prost(bytes, optional, tag="2")]
    pub file_sha256: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(bytes, optional, tag="3")]
    pub file_enc_sha256: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(bytes, optional, tag="4")]
    pub media_key: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(string, optional, tag="5")]
    pub mimetype: ::std::option::Option<std::string::String>,
    #[prost(uint32, optional, tag="6")]
    pub height: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="7")]
    pub width: ::std::option::Option<u32>,
    #[prost(string, optional, tag="8")]
    pub direct_path: ::std::option::Option<std::string::String>,
    #[prost(uint64, optional, tag="9")]
    pub file_length: ::std::option::Option<u64>,
    #[prost(int64, optional, tag="10")]
    pub media_key_timestamp: ::std::option::Option<i64>,
    #[prost(uint32, optional, tag="11")]
    pub first_frame_length: ::std::option::Option<u32>,
    #[prost(bytes, optional, tag="12")]
    pub first_frame_sidecar: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(bool, optional, tag="13")]
    pub is_animated: ::std::option::Option<bool>,
    #[prost(bytes, optional, tag="16")]
    pub png_thumbnail: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(message, optional, boxed, tag="17")]
    pub context_info: ::std::option::Option<::std::boxed::Box<ContextInfo>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FourRowTemplate {
    #[prost(message, optional, boxed, tag="6")]
    pub content: ::std::option::Option<::std::boxed::Box<HighlyStructuredMessage>>,
    #[prost(message, optional, boxed, tag="7")]
    pub footer: ::std::option::Option<::std::boxed::Box<HighlyStructuredMessage>>,
    #[prost(message, repeated, tag="8")]
    pub buttons: ::std::vec::Vec<TemplateButton>,
    #[prost(oneof="four_row_template::Title", tags="1, 2, 3, 4, 5")]
    pub title: ::std::option::Option<four_row_template::Title>,
}
pub mod four_row_template {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Title {
        #[prost(message, tag="1")]
        DocumentMessage(Box<super::DocumentMessage>),
        #[prost(message, tag="2")]
        HighlyStructuredMessage(Box<super::HighlyStructuredMessage>),
        #[prost(message, tag="3")]
        ImageMessage(Box<super::ImageMessage>),
        #[prost(message, tag="4")]
        VideoMessage(Box<super::VideoMessage>),
        #[prost(message, tag="5")]
        LocationMessage(Box<super::LocationMessage>),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HydratedFourRowTemplate {
    #[prost(string, optional, tag="6")]
    pub hydrated_content_text: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="7")]
    pub hydrated_footer_text: ::std::option::Option<std::string::String>,
    #[prost(message, repeated, tag="8")]
    pub hydrated_buttons: ::std::vec::Vec<HydratedTemplateButton>,
    #[prost(string, optional, tag="9")]
    pub template_id: ::std::option::Option<std::string::String>,
    #[prost(oneof="hydrated_four_row_template::Title", tags="1, 2, 3, 4, 5")]
    pub title: ::std::option::Option<hydrated_four_row_template::Title>,
}
pub mod hydrated_four_row_template {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Title {
        #[prost(message, tag="1")]
        DocumentMessage(Box<super::DocumentMessage>),
        #[prost(string, tag="2")]
        HydratedTitleText(std::string::String),
        #[prost(message, tag="3")]
        ImageMessage(Box<super::ImageMessage>),
        #[prost(message, tag="4")]
        VideoMessage(Box<super::VideoMessage>),
        #[prost(message, tag="5")]
        LocationMessage(Box<super::LocationMessage>),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TemplateMessage {
    #[prost(message, optional, boxed, tag="3")]
    pub context_info: ::std::option::Option<::std::boxed::Box<ContextInfo>>,
    #[prost(message, optional, boxed, tag="4")]
    pub hydrated_template: ::std::option::Option<::std::boxed::Box<HydratedFourRowTemplate>>,
    #[prost(oneof="template_message::Format", tags="1, 2")]
    pub format: ::std::option::Option<template_message::Format>,
}
pub mod template_message {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Format {
        #[prost(message, tag="1")]
        FourRowTemplate(Box<super::FourRowTemplate>),
        #[prost(message, tag="2")]
        HydratedFourRowTemplate(Box<super::HydratedFourRowTemplate>),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TemplateButtonReplyMessage {
    #[prost(string, optional, tag="1")]
    pub selected_id: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="2")]
    pub selected_display_text: ::std::option::Option<std::string::String>,
    #[prost(message, optional, boxed, tag="3")]
    pub context_info: ::std::option::Option<::std::boxed::Box<ContextInfo>>,
    #[prost(uint32, optional, tag="4")]
    pub selected_index: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CatalogSnapshot {
    #[prost(message, optional, boxed, tag="1")]
    pub catalog_image: ::std::option::Option<::std::boxed::Box<ImageMessage>>,
    #[prost(string, optional, tag="2")]
    pub title: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="3")]
    pub description: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductSnapshot {
    #[prost(message, optional, boxed, tag="1")]
    pub product_image: ::std::option::Option<::std::boxed::Box<ImageMessage>>,
    #[prost(string, optional, tag="2")]
    pub product_id: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="3")]
    pub title: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="4")]
    pub description: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="5")]
    pub currency_code: ::std::option::Option<std::string::String>,
    #[prost(int64, optional, tag="6")]
    pub price_amount1000: ::std::option::Option<i64>,
    #[prost(string, optional, tag="7")]
    pub retailer_id: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="8")]
    pub url: ::std::option::Option<std::string::String>,
    #[prost(uint32, optional, tag="9")]
    pub product_image_count: ::std::option::Option<u32>,
    #[prost(string, optional, tag="11")]
    pub first_image_id: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductMessage {
    #[prost(message, optional, boxed, tag="1")]
    pub product: ::std::option::Option<::std::boxed::Box<ProductSnapshot>>,
    #[prost(string, optional, tag="2")]
    pub business_owner_jid: ::std::option::Option<std::string::String>,
    #[prost(message, optional, boxed, tag="4")]
    pub catalog: ::std::option::Option<::std::boxed::Box<CatalogSnapshot>>,
    #[prost(message, optional, boxed, tag="17")]
    pub context_info: ::std::option::Option<::std::boxed::Box<ContextInfo>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupInviteMessage {
    #[prost(string, optional, tag="1")]
    pub group_jid: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="2")]
    pub invite_code: ::std::option::Option<std::string::String>,
    #[prost(int64, optional, tag="3")]
    pub invite_expiration: ::std::option::Option<i64>,
    #[prost(string, optional, tag="4")]
    pub group_name: ::std::option::Option<std::string::String>,
    #[prost(bytes, optional, tag="5")]
    pub jpeg_thumbnail: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(string, optional, tag="6")]
    pub caption: ::std::option::Option<std::string::String>,
    #[prost(message, optional, boxed, tag="7")]
    pub context_info: ::std::option::Option<::std::boxed::Box<ContextInfo>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceSentMessage {
    #[prost(string, optional, tag="1")]
    pub destination_jid: ::std::option::Option<std::string::String>,
    #[prost(message, optional, boxed, tag="2")]
    pub message: ::std::option::Option<::std::boxed::Box<Message>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceSyncMessage {
    #[prost(bytes, optional, tag="1")]
    pub serialized_xml_bytes: ::std::option::Option<std::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    #[prost(string, optional, tag="1")]
    pub conversation: ::std::option::Option<std::string::String>,
    #[prost(message, optional, tag="2")]
    pub sender_key_distribution_message: ::std::option::Option<SenderKeyDistributionMessage>,
    #[prost(message, optional, boxed, tag="3")]
    pub image_message: ::std::option::Option<::std::boxed::Box<ImageMessage>>,
    #[prost(message, optional, boxed, tag="4")]
    pub contact_message: ::std::option::Option<::std::boxed::Box<ContactMessage>>,
    #[prost(message, optional, boxed, tag="5")]
    pub location_message: ::std::option::Option<::std::boxed::Box<LocationMessage>>,
    #[prost(message, optional, boxed, tag="6")]
    pub extended_text_message: ::std::option::Option<::std::boxed::Box<ExtendedTextMessage>>,
    #[prost(message, optional, boxed, tag="7")]
    pub document_message: ::std::option::Option<::std::boxed::Box<DocumentMessage>>,
    #[prost(message, optional, boxed, tag="8")]
    pub audio_message: ::std::option::Option<::std::boxed::Box<AudioMessage>>,
    #[prost(message, optional, boxed, tag="9")]
    pub video_message: ::std::option::Option<::std::boxed::Box<VideoMessage>>,
    #[prost(message, optional, tag="10")]
    pub call: ::std::option::Option<Call>,
    #[prost(message, optional, tag="11")]
    pub chat: ::std::option::Option<Chat>,
    #[prost(message, optional, tag="12")]
    pub protocol_message: ::std::option::Option<ProtocolMessage>,
    #[prost(message, optional, boxed, tag="13")]
    pub contacts_array_message: ::std::option::Option<::std::boxed::Box<ContactsArrayMessage>>,
    #[prost(message, optional, boxed, tag="14")]
    pub highly_structured_message: ::std::option::Option<::std::boxed::Box<HighlyStructuredMessage>>,
    #[prost(message, optional, tag="15")]
    pub fast_ratchet_key_sender_key_distribution_message: ::std::option::Option<SenderKeyDistributionMessage>,
    #[prost(message, optional, boxed, tag="16")]
    pub send_payment_message: ::std::option::Option<::std::boxed::Box<SendPaymentMessage>>,
    #[prost(message, optional, boxed, tag="18")]
    pub live_location_message: ::std::option::Option<::std::boxed::Box<LiveLocationMessage>>,
    #[prost(message, optional, boxed, tag="22")]
    pub request_payment_message: ::std::option::Option<::std::boxed::Box<RequestPaymentMessage>>,
    #[prost(message, optional, tag="23")]
    pub decline_payment_request_message: ::std::option::Option<DeclinePaymentRequestMessage>,
    #[prost(message, optional, tag="24")]
    pub cancel_payment_request_message: ::std::option::Option<CancelPaymentRequestMessage>,
    #[prost(message, optional, boxed, tag="25")]
    pub template_message: ::std::option::Option<::std::boxed::Box<TemplateMessage>>,
    #[prost(message, optional, boxed, tag="26")]
    pub sticker_message: ::std::option::Option<::std::boxed::Box<StickerMessage>>,
    #[prost(message, optional, boxed, tag="28")]
    pub group_invite_message: ::std::option::Option<::std::boxed::Box<GroupInviteMessage>>,
    #[prost(message, optional, boxed, tag="29")]
    pub template_button_reply_message: ::std::option::Option<::std::boxed::Box<TemplateButtonReplyMessage>>,
    #[prost(message, optional, boxed, tag="30")]
    pub product_message: ::std::option::Option<::std::boxed::Box<ProductMessage>>,
    #[prost(message, optional, boxed, tag="31")]
    pub device_sent_message: ::std::option::Option<::std::boxed::Box<DeviceSentMessage>>,
    #[prost(message, optional, tag="32")]
    pub device_sync_message: ::std::option::Option<DeviceSyncMessage>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageKey {
    #[prost(string, optional, tag="1")]
    pub remote_jid: ::std::option::Option<std::string::String>,
    #[prost(bool, optional, tag="2")]
    pub from_me: ::std::option::Option<bool>,
    #[prost(string, optional, tag="3")]
    pub id: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="4")]
    pub participant: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebFeatures {
    #[prost(enumeration="web_features::WebFeaturesFlag", optional, tag="1")]
    pub labels_display: ::std::option::Option<i32>,
    #[prost(enumeration="web_features::WebFeaturesFlag", optional, tag="2")]
    pub voip_individual_outgoing: ::std::option::Option<i32>,
    #[prost(enumeration="web_features::WebFeaturesFlag", optional, tag="3")]
    pub groups_v3: ::std::option::Option<i32>,
    #[prost(enumeration="web_features::WebFeaturesFlag", optional, tag="4")]
    pub groups_v3_create: ::std::option::Option<i32>,
    #[prost(enumeration="web_features::WebFeaturesFlag", optional, tag="5")]
    pub change_number_v2: ::std::option::Option<i32>,
    #[prost(enumeration="web_features::WebFeaturesFlag", optional, tag="6")]
    pub query_status_v3_thumbnail: ::std::option::Option<i32>,
    #[prost(enumeration="web_features::WebFeaturesFlag", optional, tag="7")]
    pub live_locations: ::std::option::Option<i32>,
    #[prost(enumeration="web_features::WebFeaturesFlag", optional, tag="8")]
    pub query_vname: ::std::option::Option<i32>,
    #[prost(enumeration="web_features::WebFeaturesFlag", optional, tag="9")]
    pub voip_individual_incoming: ::std::option::Option<i32>,
    #[prost(enumeration="web_features::WebFeaturesFlag", optional, tag="10")]
    pub quick_replies_query: ::std::option::Option<i32>,
    #[prost(enumeration="web_features::WebFeaturesFlag", optional, tag="11")]
    pub payments: ::std::option::Option<i32>,
    #[prost(enumeration="web_features::WebFeaturesFlag", optional, tag="12")]
    pub sticker_pack_query: ::std::option::Option<i32>,
    #[prost(enumeration="web_features::WebFeaturesFlag", optional, tag="13")]
    pub live_locations_final: ::std::option::Option<i32>,
    #[prost(enumeration="web_features::WebFeaturesFlag", optional, tag="14")]
    pub labels_edit: ::std::option::Option<i32>,
    #[prost(enumeration="web_features::WebFeaturesFlag", optional, tag="15")]
    pub media_upload: ::std::option::Option<i32>,
    #[prost(enumeration="web_features::WebFeaturesFlag", optional, tag="18")]
    pub media_upload_rich_quick_replies: ::std::option::Option<i32>,
    #[prost(enumeration="web_features::WebFeaturesFlag", optional, tag="19")]
    pub vname_v2: ::std::option::Option<i32>,
    #[prost(enumeration="web_features::WebFeaturesFlag", optional, tag="20")]
    pub video_playback_url: ::std::option::Option<i32>,
    #[prost(enumeration="web_features::WebFeaturesFlag", optional, tag="21")]
    pub status_ranking: ::std::option::Option<i32>,
    #[prost(enumeration="web_features::WebFeaturesFlag", optional, tag="22")]
    pub voip_individual_video: ::std::option::Option<i32>,
    #[prost(enumeration="web_features::WebFeaturesFlag", optional, tag="23")]
    pub third_party_stickers: ::std::option::Option<i32>,
    #[prost(enumeration="web_features::WebFeaturesFlag", optional, tag="24")]
    pub frequently_forwarded_setting: ::std::option::Option<i32>,
    #[prost(enumeration="web_features::WebFeaturesFlag", optional, tag="25")]
    pub groups_v4_join_permission: ::std::option::Option<i32>,
    #[prost(enumeration="web_features::WebFeaturesFlag", optional, tag="26")]
    pub recent_stickers: ::std::option::Option<i32>,
    #[prost(enumeration="web_features::WebFeaturesFlag", optional, tag="27")]
    pub catalog: ::std::option::Option<i32>,
    #[prost(enumeration="web_features::WebFeaturesFlag", optional, tag="28")]
    pub starred_stickers: ::std::option::Option<i32>,
    #[prost(enumeration="web_features::WebFeaturesFlag", optional, tag="29")]
    pub voip_group_call: ::std::option::Option<i32>,
    #[prost(enumeration="web_features::WebFeaturesFlag", optional, tag="30")]
    pub template_message: ::std::option::Option<i32>,
    #[prost(enumeration="web_features::WebFeaturesFlag", optional, tag="31")]
    pub template_message_interactivity: ::std::option::Option<i32>,
    #[prost(enumeration="web_features::WebFeaturesFlag", optional, tag="32")]
    pub ephemeral_messages: ::std::option::Option<i32>,
    #[prost(enumeration="web_features::WebFeaturesFlag", optional, tag="33")]
    pub e2_e_notification_sync: ::std::option::Option<i32>,
    #[prost(enumeration="web_features::WebFeaturesFlag", optional, tag="34")]
    pub recent_stickers_v2: ::std::option::Option<i32>,
}
pub mod web_features {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum WebFeaturesFlag {
        NotStarted = 0,
        ForceUpgrade = 1,
        Development = 2,
        Production = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TabletNotificationsInfo {
    #[prost(uint64, optional, tag="2")]
    pub timestamp: ::std::option::Option<u64>,
    #[prost(uint32, optional, tag="3")]
    pub unread_chats: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="4")]
    pub notify_message_count: ::std::option::Option<u32>,
    #[prost(message, repeated, tag="5")]
    pub notify_message: ::std::vec::Vec<NotificationMessageInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationMessageInfo {
    #[prost(message, optional, tag="1")]
    pub key: ::std::option::Option<MessageKey>,
    #[prost(message, optional, tag="2")]
    pub message: ::std::option::Option<Message>,
    #[prost(uint64, optional, tag="3")]
    pub message_timestamp: ::std::option::Option<u64>,
    #[prost(string, optional, tag="4")]
    pub participant: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebNotificationsInfo {
    #[prost(uint64, optional, tag="2")]
    pub timestamp: ::std::option::Option<u64>,
    #[prost(uint32, optional, tag="3")]
    pub unread_chats: ::std::option::Option<u32>,
    #[prost(uint32, optional, tag="4")]
    pub notify_message_count: ::std::option::Option<u32>,
    #[prost(message, repeated, tag="5")]
    pub notify_messages: ::std::vec::Vec<WebMessageInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentInfo {
    #[prost(enumeration="payment_info::PaymentInfoCurrency", optional, tag="1")]
    pub currency_deprecated: ::std::option::Option<i32>,
    #[prost(uint64, optional, tag="2")]
    pub amount1000: ::std::option::Option<u64>,
    #[prost(string, optional, tag="3")]
    pub receiver_jid: ::std::option::Option<std::string::String>,
    #[prost(enumeration="payment_info::PaymentInfoStatus", optional, tag="4")]
    pub status: ::std::option::Option<i32>,
    #[prost(uint64, optional, tag="5")]
    pub transaction_timestamp: ::std::option::Option<u64>,
    #[prost(message, optional, tag="6")]
    pub request_message_key: ::std::option::Option<MessageKey>,
    #[prost(uint64, optional, tag="7")]
    pub expiry_timestamp: ::std::option::Option<u64>,
    #[prost(bool, optional, tag="8")]
    pub futureproofed: ::std::option::Option<bool>,
    #[prost(string, optional, tag="9")]
    pub currency: ::std::option::Option<std::string::String>,
    #[prost(enumeration="payment_info::PaymentInfoTxnstatus", optional, tag="10")]
    pub txn_status: ::std::option::Option<i32>,
}
pub mod payment_info {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PaymentInfoCurrency {
        UnknownCurrency = 0,
        Inr = 1,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PaymentInfoStatus {
        UnknownStatus = 0,
        Processing = 1,
        Sent = 2,
        NeedToAccept = 3,
        Complete = 4,
        CouldNotComplete = 5,
        Refunded = 6,
        Expired = 7,
        Rejected = 8,
        Cancelled = 9,
        WaitingForPayer = 10,
        Waiting = 11,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PaymentInfoTxnstatus {
        Unknown = 0,
        PendingSetup = 1,
        PendingReceiverSetup = 2,
        Init = 3,
        Success = 4,
        Completed = 5,
        Failed = 6,
        FailedRisk = 7,
        FailedProcessing = 8,
        FailedReceiverProcessing = 9,
        FailedDa = 10,
        FailedDaFinal = 11,
        RefundedTxn = 12,
        RefundFailed = 13,
        RefundFailedProcessing = 14,
        RefundFailedDa = 15,
        ExpiredTxn = 16,
        AuthCanceled = 17,
        AuthCancelFailedProcessing = 18,
        AuthCancelFailed = 19,
        CollectInit = 20,
        CollectSuccess = 21,
        CollectFailed = 22,
        CollectFailedRisk = 23,
        CollectRejected = 24,
        CollectExpired = 25,
        CollectCanceled = 26,
        CollectCancelling = 27,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebMessageInfo {
    #[prost(message, required, tag="1")]
    pub key: MessageKey,
    #[prost(message, optional, tag="2")]
    pub message: ::std::option::Option<Message>,
    #[prost(uint64, optional, tag="3")]
    pub message_timestamp: ::std::option::Option<u64>,
    #[prost(enumeration="web_message_info::WebMessageInfoStatus", optional, tag="4")]
    pub status: ::std::option::Option<i32>,
    #[prost(string, optional, tag="5")]
    pub participant: ::std::option::Option<std::string::String>,
    #[prost(bool, optional, tag="16")]
    pub ignore: ::std::option::Option<bool>,
    #[prost(bool, optional, tag="17")]
    pub starred: ::std::option::Option<bool>,
    #[prost(bool, optional, tag="18")]
    pub broadcast: ::std::option::Option<bool>,
    #[prost(string, optional, tag="19")]
    pub push_name: ::std::option::Option<std::string::String>,
    #[prost(bytes, optional, tag="20")]
    pub media_ciphertext_sha256: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(bool, optional, tag="21")]
    pub multicast: ::std::option::Option<bool>,
    #[prost(bool, optional, tag="22")]
    pub url_text: ::std::option::Option<bool>,
    #[prost(bool, optional, tag="23")]
    pub url_number: ::std::option::Option<bool>,
    #[prost(enumeration="web_message_info::WebMessageInfoStubtype", optional, tag="24")]
    pub message_stub_type: ::std::option::Option<i32>,
    #[prost(bool, optional, tag="25")]
    pub clear_media: ::std::option::Option<bool>,
    #[prost(string, repeated, tag="26")]
    pub message_stub_parameters: ::std::vec::Vec<std::string::String>,
    #[prost(uint32, optional, tag="27")]
    pub duration: ::std::option::Option<u32>,
    #[prost(string, repeated, tag="28")]
    pub labels: ::std::vec::Vec<std::string::String>,
    #[prost(message, optional, tag="29")]
    pub payment_info: ::std::option::Option<PaymentInfo>,
    #[prost(message, optional, tag="30")]
    pub final_live_location: ::std::option::Option<LiveLocationMessage>,
    #[prost(message, optional, tag="31")]
    pub quoted_payment_info: ::std::option::Option<PaymentInfo>,
    #[prost(uint64, optional, tag="32")]
    pub ephemeral_start_timestamp: ::std::option::Option<u64>,
    #[prost(uint32, optional, tag="33")]
    pub ephemeral_duration: ::std::option::Option<u32>,
}
pub mod web_message_info {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum WebMessageInfoStatus {
        Error = 0,
        Pending = 1,
        ServerAck = 2,
        DeliveryAck = 3,
        Read = 4,
        Played = 5,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum WebMessageInfoStubtype {
        Unknown = 0,
        Revoke = 1,
        Ciphertext = 2,
        Futureproof = 3,
        NonVerifiedTransition = 4,
        UnverifiedTransition = 5,
        VerifiedTransition = 6,
        VerifiedLowUnknown = 7,
        VerifiedHigh = 8,
        VerifiedInitialUnknown = 9,
        VerifiedInitialLow = 10,
        VerifiedInitialHigh = 11,
        VerifiedTransitionAnyToNone = 12,
        VerifiedTransitionAnyToHigh = 13,
        VerifiedTransitionHighToLow = 14,
        VerifiedTransitionHighToUnknown = 15,
        VerifiedTransitionUnknownToLow = 16,
        VerifiedTransitionLowToUnknown = 17,
        VerifiedTransitionNoneToLow = 18,
        VerifiedTransitionNoneToUnknown = 19,
        GroupCreate = 20,
        GroupChangeSubject = 21,
        GroupChangeIcon = 22,
        GroupChangeInviteLink = 23,
        GroupChangeDescription = 24,
        GroupChangeRestrict = 25,
        GroupChangeAnnounce = 26,
        GroupParticipantAdd = 27,
        GroupParticipantRemove = 28,
        GroupParticipantPromote = 29,
        GroupParticipantDemote = 30,
        GroupParticipantInvite = 31,
        GroupParticipantLeave = 32,
        GroupParticipantChangeNumber = 33,
        BroadcastCreate = 34,
        BroadcastAdd = 35,
        BroadcastRemove = 36,
        GenericNotification = 37,
        E2eIdentityChanged = 38,
        E2eEncrypted = 39,
        CallMissedVoice = 40,
        CallMissedVideo = 41,
        IndividualChangeNumber = 42,
        GroupDelete = 43,
        GroupAnnounceModeMessageBounce = 44,
        CallMissedGroupVoice = 45,
        CallMissedGroupVideo = 46,
        PaymentCiphertext = 47,
        PaymentFutureproof = 48,
        PaymentTransactionStatusUpdateFailed = 49,
        PaymentTransactionStatusUpdateRefunded = 50,
        PaymentTransactionStatusUpdateRefundFailed = 51,
        PaymentTransactionStatusReceiverPendingSetup = 52,
        PaymentTransactionStatusReceiverSuccessAfterHiccup = 53,
        PaymentActionAccountSetupReminder = 54,
        PaymentActionSendPaymentReminder = 55,
        PaymentActionSendPaymentInvitation = 56,
        PaymentActionRequestDeclined = 57,
        PaymentActionRequestExpired = 58,
        PaymentActionRequestCancelled = 59,
        BizVerifiedTransitionTopToBottom = 60,
        BizVerifiedTransitionBottomToTop = 61,
        BizIntroTop = 62,
        BizIntroBottom = 63,
        BizNameChange = 64,
        BizMoveToConsumerApp = 65,
        BizTwoTierMigrationTop = 66,
        BizTwoTierMigrationBottom = 67,
        Oversized = 68,
        GroupChangeNoFrequentlyForwarded = 69,
        GroupV4AddInviteSent = 70,
        GroupParticipantAddRequestJoin = 71,
        ChangeEphemeralSetting = 72,
    }
}
