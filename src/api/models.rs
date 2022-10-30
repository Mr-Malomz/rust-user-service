use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub avatar: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Records {
    pub records: Vec<User>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateResponse {
    pub id: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct APIResponse<T> {
    pub status: u16,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Serialize, Debug, Clone)]
pub struct APIErrorResponse {
    pub status: u16,
    pub message: String,
    pub data: Option<String>,
}
