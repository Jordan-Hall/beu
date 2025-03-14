use crate::{
    app_error::AppError,
    db::models::user::{AuthenticatedUser, SlimUser, User, UserFindableCol},
};
use actix_identity::Identity;
use actix_session::Session;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use surrealdb::{engine::remote::ws::Client, Surreal};

#[derive(Deserialize, Debug)]
pub struct SignInBody {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: String,
}

/// Checks and returns if the user is using username or email to login
fn get_auth_cred_method(
    username: Option<String>,
    email: Option<String>,
) -> Result<(UserFindableCol, String), AppError> {
    if username.is_none() && email.is_none() {
        return Err(AppError::BadRequest(
            "You need to provide either the 'username' or 'email'".into(),
        ));
    }

    let method = match (username, email) {
        (Some(username), _) => (UserFindableCol::Username, username),
        (_, Some(email)) => (UserFindableCol::Email, email),
        _ => unreachable!(), // You've already checked for both being None above
    };
    Ok(method)
}

fn match_user_exists(
    user: Option<User>,
    auth_method: UserFindableCol,
    value: String,
) -> Result<User, AppError> {
    match user {
        Some(user) => Ok(user),
        None => {
            let method: String = auth_method.into();
            return Err(AppError::BadRequest(format!(
                "User with {} {} not found...",
                method, value
            )));
        }
    }
}

pub async fn login(
    db: web::Data<Surreal<Client>>,
    body: web::Json<SignInBody>,
    session: Session,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let username = body.username.to_owned();
    let email = body.email.to_owned();
    let password = body.password.to_owned();

    // user can either log in with the username or email
    let (auth_method, value) = get_auth_cred_method(username, email)?;

    // Find the user from the database
    let user = User::find_one(&db, auth_method.clone(), value.clone()).await?;

    // Match If the user exists in the database
    let user = match_user_exists(user, auth_method, value)?;

    let check_password = user.verify_password(&password).await?;

    match check_password {
        true => {
            let slim_user = SlimUser::from(&user);
            let user_store = AuthenticatedUser::from(&user);
            // Login with identity middleware
            Identity::login(&req.extensions(), slim_user.id.clone())?;

            // Add the Simplified user to the session
            session.insert(&crate::APP_CONFIG.auth_cookie_key, user_store)?;

            Ok(HttpResponse::Ok().json(json!(
            {"status": "success",
            "msg": "logging_in",
            "user": slim_user,
            })))
        }
        false => {
            return Err(AppError::BadRequest("Provided Incorrect Password..".into()));
        }
    }
}
