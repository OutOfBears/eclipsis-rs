/*
 * xethlyx's api server
 *
 * Public APIs for use within whatever you want. These are ratelimited, so try to keep requests to a minimum.  Future use of the API will require an API key.  ## API Keys API keys need to be passed alongside every request. With an API key, you are also expected to adhere to the following guidelines:   - Do not share your API key.   - Data can be retained for a maximum of 30 days (this is to adhere with GDPR guidelines).  API keys can also be saved to the browser for convenience using the client login/logout APIs.  You can obtain an API key by asking staff in the [Eclipsis discord](https://discord.gg/AkDsUtz).
 *
 * The version of the OpenAPI document: 3.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Match {
    /// Discord's ID format. To get a snowflake, turn on Developer Mode and click Copy ID from any context menu.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// How long the match took in seconds
    #[serde(rename = "match_time", skip_serializing_if = "Option::is_none")]
    pub match_time: Option<i64>,
    /// The type of match
    #[serde(rename = "match_type", skip_serializing_if = "Option::is_none")]
    pub match_type: Option<MatchType>,
    #[serde(rename = "teams", skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<crate::models::MatchTeam>>,
}

impl Match {
    pub fn new() -> Match {
        Match {
            id: None,
            match_time: None,
            match_type: None,
            teams: None,
        }
    }
}

/// The type of match
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MatchType {
    #[serde(rename = "TeamRated")]
    TeamRated,
    #[serde(rename = "TeamUnrated")]
    TeamUnrated,
    #[serde(rename = "SoloRated")]
    SoloRated,
    #[serde(rename = "SoloUnrated")]
    SoloUnrated,
}

impl Default for MatchType {
    fn default() -> MatchType {
        Self::TeamRated
    }
}