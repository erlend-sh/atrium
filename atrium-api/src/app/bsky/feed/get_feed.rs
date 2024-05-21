// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `app.bsky.feed.getFeed` namespace.
pub const NSID: &str = "app.bsky.feed.getFeed";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    pub feed: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<crate::types::LimitedNonZeroU8<100u8>>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    pub feed: Vec<crate::app::bsky::feed::defs::FeedViewPost>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {
    UnknownFeed(Option<String>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::UnknownFeed(msg) => {
                write!(_f, "UnknownFeed")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
        }
        Ok(())
    }
}
