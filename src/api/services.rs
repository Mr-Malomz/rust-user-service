use super::models::{CreateResponse, Records, User};
use reqwest::{Client, Error};

pub struct UserService {}

impl UserService {
    pub async fn get_user() -> Result<Records, Error> {
        let url = "<NETLIFY FUNCTION GET URL>";

        let client = Client::new();
        let fetched_users = client.get(url).send().await;

        match fetched_users {
            Ok(response) => {
                let json = response.text().await?;
                let new_records: Records = serde_json::from_str(json.as_str()).unwrap();

                Ok(new_records)
            }
            Err(error) => Err(error),
        }
    }

    pub async fn create_user(new_user: User) -> Result<CreateResponse, Error> {
        let url = "NETLIFY FUNCTION CREATE URL";
        let json_body = User {
            id: None,
            first_name: new_user.first_name,
            last_name: new_user.last_name,
            phone_number: new_user.phone_number,
            avatar: None,
        };

        let client = Client::new();
        let fetched_users = client.post(url).json(&json_body).send().await;

        match fetched_users {
            Ok(response) => {
                let json = response.text().await?;
                let created_record: CreateResponse = serde_json::from_str(json.as_str()).unwrap();

                Ok(created_record)
            }
            Err(error) => Err(error),
        }
    }
}
