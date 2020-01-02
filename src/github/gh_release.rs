use crate::sources::{ReleaseInfo, Source, Sourceable};
use serde_derive;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Release {
    url: String,
    assets_url: String,
    upload_url: String,
    html_url: String,
    pub id: i64,
    node_id: String,
    pub tag_name: String,
    target_commitish: String,
    pub name: String,
    draft: bool,
    author: Author,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: String,
    assets: Vec<Asset>,
    pub tarball_url: String,
    pub zipball_url: String,
    body: String,
}

impl Sourceable for Release {
    fn to_source(&self) -> Source {
        let info = ReleaseInfo::new(
            &self.tag_name,
            &self.name,
            &self.prerelease,
            &self.created_at,
            &self.published_at,
            &self.tarball_url,
            &self.zipball_url,
        );
        Source::new("", "", "", &self.prerelease, info)
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct Author {
    login: String,
    id: i64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    #[serde(rename = "type")]
    type_field: String,
    site_admin: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct Asset {
    url: String,
    id: i64,
    node_id: String,
    name: String,
    label: String,
    uploader: Uploader,
    content_type: String,
    state: String,
    size: i64,
    download_count: i64,
    created_at: String,
    updated_at: String,
    browser_download_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
struct Uploader {
    login: String,
    id: i64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    #[serde(rename = "type")]
    type_field: String,
    site_admin: bool,
}
