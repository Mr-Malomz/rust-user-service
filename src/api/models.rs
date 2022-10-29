use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
}


#[derive(Deserialize, Debug, Clone)]
pub struct Records {
    pub records: Vec<User>
}

#[derive(Serialize, Debug, Clone)]
pub struct APIResponse {
    pub status: u16,
    pub message: String,
    pub data: String,
}