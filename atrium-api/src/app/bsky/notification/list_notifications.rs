// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `app.bsky.notification.listNotifications` namespace.
pub const NSID: &str = "app.bsky.notification.listNotifications";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ParametersData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<crate::types::LimitedNonZeroU8<100u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seen_at: Option<crate::types::string::Datetime>,
}
pub type Parameters = crate::types::Object<ParametersData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OutputData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    pub notifications: Vec<Notification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seen_at: Option<crate::types::string::Datetime>,
}
pub type Output = crate::types::Object<OutputData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(())
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NotificationData {
    pub author: crate::app::bsky::actor::defs::ProfileView,
    pub cid: crate::types::string::Cid,
    pub indexed_at: crate::types::string::Datetime,
    pub is_read: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::com::atproto::label::defs::Label>>,
    ///Expected values are 'like', 'repost', 'follow', 'mention', 'reply', 'quote', and 'starterpack-joined'.
    pub reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_subject: Option<String>,
    pub record: crate::types::Unknown,
    pub uri: String,
}
pub type Notification = crate::types::Object<NotificationData>;
