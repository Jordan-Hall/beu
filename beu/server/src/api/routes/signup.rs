use actix_web::{web, HttpResponse};
use serde_json::json;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{app_error::AppError, db::models::user::User};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UserCreationBody {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub email: String,
}

pub async fn register(
    body: web::Json<UserCreationBody>,
    db: web::Data<Surreal<Client>>,
) -> Result<HttpResponse, AppError> {
    let username = body.username.clone();
    let first_name = body.first_name.clone();
    let last_name = body.last_name.clone();
    let password = body.password.clone();

    let email: String = body.email.clone();
    let age = body.age;

    let mut new_user: User = User {
        id: None,
        first_name,
        last_name,
        password,
        is_admin: false,
        created_at: chrono::Utc::now().into(),
        email,
    };

    // Hashing of password id done internally in the create function after user existing check
    let new_user = new_user.create(&db).await?;

    match new_user {
        Some(user) => {
            let response_body = json!({
                "status": 201,
                "message": "User Created",
                "user": user,
            });
            Ok(HttpResponse::Created().json(response_body))
        }
        None => Ok(HttpResponse::Ok().finish()),
    }
}