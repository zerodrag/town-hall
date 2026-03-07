use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};
use specta::Type;

#[serde_as]
#[derive(Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    #[serde_as(as = "DisplayFromStr")]
    pub github_id: i64,
    pub username: String,
}

#[serde_as]
#[derive(Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRequest {
    #[serde_as(as = "DisplayFromStr")]
    pub github_id: i64,
    pub username: String,
}
