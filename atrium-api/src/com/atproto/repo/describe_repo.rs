// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `com.atproto.repo.describeRepo` namespace.
pub const NSID: &str = "com.atproto.repo.describeRepo";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    ///The handle or DID of the repo.
    pub repo: crate::types::string::AtIdentifier,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    ///List of all the collections (NSIDs) for which this repo contains at least one record.
    pub collections: Vec<crate::types::string::Nsid>,
    pub did: crate::types::string::Did,
    ///The complete DID document for this account.
    pub did_doc: crate::did_doc::DidDocument,
    pub handle: crate::types::string::Handle,
    ///Indicates if handle is currently valid (resolves bi-directionally)
    pub handle_is_correct: bool,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(())
    }
}
