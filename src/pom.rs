use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Pom {
    pub repositories: Option<Repositories>,
    #[serde(rename = "distributionManagement")]
    pub distribution_management: Option<Repositories>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Repositories {
    #[serde(rename = "repository", default)]
    pub repositories: Vec<Repository>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Repository {
    pub id: String,
    pub url: String,
}

