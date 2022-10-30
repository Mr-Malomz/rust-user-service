use actix_web::{get, post, web::Json, HttpResponse};
use reqwest::StatusCode;

use super::{
    models::{APIErrorResponse, APIResponse, CreateResponse, Records, User},
    services::UserService,
};

#[get("/users")]
pub async fn get_user() -> HttpResponse {
    let user_details = UserService::get_user().await;

    match user_details {
        Ok(data) => HttpResponse::Ok().json(APIResponse::<Records> {
            status: StatusCode::OK.as_u16(),
            message: "success".to_string(),
            data: Some(data),
        }),
        Err(error) => HttpResponse::InternalServerError().json(APIErrorResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: "failure".to_string(),
            data: Some(error.to_string()),
        }),
    }
}

#[post("/users")]
pub async fn create_user(data: Json<User>) -> HttpResponse {
    let new_user = User {
        id: None,
        first_name: data.first_name.clone(),
        last_name: data.last_name.clone(),
        phone_number: data.phone_number.clone(),
        avatar: None,
    };
    let user_details = UserService::create_user(new_user).await;

    match user_details {
        Ok(data) => HttpResponse::Accepted().json(APIResponse::<CreateResponse> {
            status: StatusCode::ACCEPTED.as_u16(),
            message: "success".to_string(),
            data: Some(data),
        }),
        Err(error) => HttpResponse::InternalServerError().json(APIErrorResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: "failure".to_string(),
            data: Some(error.to_string()),
        }),
    }
}
