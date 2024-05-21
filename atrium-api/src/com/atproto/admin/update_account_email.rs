// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `com.atproto.admin.updateAccountEmail` namespace.
pub const NSID: &str = "com.atproto.admin.updateAccountEmail";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    ///The handle or DID of the repo.
    pub account: crate::types::string::AtIdentifier,
    pub email: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(())
    }
}
