// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `com.atproto.admin.updateAccountHandle` namespace.
pub const NSID: &str = "com.atproto.admin.updateAccountHandle";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub did: crate::types::string::Did,
    pub handle: crate::types::string::Handle,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(())
    }
}
