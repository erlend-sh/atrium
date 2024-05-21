// This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `app.bsky.unspecced.searchPostsSkeleton` namespace.
pub const NSID: &str = "app.bsky.unspecced.searchPostsSkeleton";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    ///Filter to posts by the given account. Handles are resolved to DID before query-time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<crate::types::string::AtIdentifier>,
    ///Optional pagination mechanism; may not necessarily allow scrolling through entire result set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    ///Filter to posts with URLs (facet links or embeds) linking to the given domain (hostname). Server may apply hostname normalization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    ///Filter to posts in the given language. Expected to be based on post language field, though server may override language detection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<crate::types::string::Language>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<crate::types::LimitedNonZeroU8<100u8>>,
    ///Filter to posts which mention the given account. Handles are resolved to DID before query-time. Only matches rich-text facet mentions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<crate::types::string::AtIdentifier>,
    ///Search query string; syntax, phrase, boolean, and faceting is unspecified, but Lucene query syntax is recommended.
    pub q: String,
    ///Filter results for posts after the indicated datetime (inclusive). Expected to use 'sortAt' timestamp, which may not match 'createdAt'. Can be a datetime, or just an ISO date (YYYY-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    ///Specifies the ranking order of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    ///Filter to posts with the given tag (hashtag), based on rich-text facet or tag field. Do not include the hash (#) prefix. Multiple tags can be specified, with 'AND' matching.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<String>>,
    ///Filter results for posts before the indicated datetime (not inclusive). Expected to use 'sortAt' timestamp, which may not match 'createdAt'. Can be a datetime, or just an ISO date (YYY-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
    ///Filter to posts with links (facet links or embeds) pointing to this URL. Server may apply URL normalization or fuzzy matching.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    ///DID of the account making the request (not included for public/unauthenticated queries). Used for 'from:me' queries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer: Option<crate::types::string::Did>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    ///Count of search hits. Optional, may be rounded/truncated, and may not be possible to paginate through all hits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hits_total: Option<i64>,
    pub posts: Vec<crate::app::bsky::unspecced::defs::SkeletonSearchPost>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {
    BadQueryString(Option<String>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::BadQueryString(msg) => {
                write!(_f, "BadQueryString")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
        }
        Ok(())
    }
}
