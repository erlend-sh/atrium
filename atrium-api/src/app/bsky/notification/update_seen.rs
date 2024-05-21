// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `app.bsky.notification.updateSeen` namespace.
pub const NSID: &str = "app.bsky.notification.updateSeen";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub seen_at: crate::types::string::Datetime,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(())
    }
}
