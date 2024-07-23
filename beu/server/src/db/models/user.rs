use actix_web::web::{Data, ReqData};
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::Client,
    sql::{Datetime, Thing, Value},
    Surreal,
};
use crate::utils::password::password;

use crate::app_error::AppError;

const TABLE_NAME: &str = "user";

pub enum UserExists {
    Username,
    Email,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Option<Thing>,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub created_at: Datetime,
    pub is_admin: bool,
}

impl From<User> for Value {
    fn from(user: User) -> Self {
        let mut user_map = crate::data_map![
            "email" => user.email.into(),
            "password" => user.password.into(),
            "first_name" => user.first_name.into(),
            "last_name" => user.last_name.into(),
            "created_at" => user.created_at.into(),
            "is_admin" => user.is_admin.into(),
        ];

        if let Some(id) = user.id {
            user_map.insert("id".into(), id.into());
        }

        user_map.into()
    }
}

impl User {
    pub async fn get_all(db: &Data<Surreal<Client>>) -> Result<Vec<User>, AppError> {
        let q = "SELECT * from type::table($tb);";
        let mut response = db.query(q).bind(("tb", TABLE_NAME)).await?;
        let users = response.take::<Vec<User>>(0)?;
        Ok(users)
    }

    pub async fn create(&mut self, db: &Data<Surreal<Client>>) -> Result<Option<User>, AppError> {
        let existing = self.exists(&db).await?;
        if let Some(existing) = existing {
            match existing {
                UserExists::Username => {
                    return Err(AppError::BadRequest(format!(
                        "The user with email '{}' already exists...",
                        self.email,
                    )))
                }
                UserExists::Email => {
                    return Err(AppError::BadRequest(format!(
                        "The user with email '{}' already exists!",
                        self.email,
                    )))
                }
            }
        }

        self.password = password::hash_password(&self.password)?;
        let q = "CREATE type::table($table) CONTENT $data RETURN *";
        let vars = crate::data_map!["table" => TABLE_NAME.into(), "data" => self.clone().into()];
        let mut db_response = db.query(q).bind(vars).await?;
        let user: Option<User> = db_response.take(0)?;
        Ok(user)
    }

    pub async fn verify_password(&self, password: &str) -> Result<bool, AppError> {
        password::verify(password, &self.password)
    }

    pub async fn find_one(
        db: &Data<Surreal<Client>>,
        search_term: UserFindableCol,
        value: impl Into<Value>,
    ) -> Result<Option<User>, AppError> {
        let search_term: String = search_term.into();
        let find_q = format!("SELECT * FROM type::table($table) where {} = $value;", search_term);
        let vars = crate::data_map![
            "table" => TABLE_NAME.into(),
            "value" => value.into()
        ];

        let mut db_res = db.query(find_q).bind(vars).await?;
        let db_res: Vec<User> = db_res.take(0)?;

        if db_res.is_empty() {
            return Ok(None);
        }
        let user = db_res[0].clone();

        Ok(Some(user))
    }

    pub async fn exists(&self, db: &Data<Surreal<Client>>) -> Result<Option<UserExists>, AppError> {
        let find_q = "SELECT email FROM type::table($table) where email = $email";

        let vars = crate::data_map![
            "table" => TABLE_NAME.into(),
            "email" => self.email.clone().into(),
        ];

        let mut db_res = db.query(find_q).bind(vars).await?;
        let existing_email: Option<String> = db_res.take("email")?;

        if let Some(email) = existing_email {
            if email == self.email {
                return Ok(Some(UserExists::Email));
            }
        }

        Ok(None)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: String,
    pub email: String,
    pub is_admin: bool,
}

impl From<&User> for AuthenticatedUser {
    fn from(user: &User) -> Self {
        AuthenticatedUser {
            id: user.id.clone().unwrap().to_string(),
            email: user.email.clone(),
            is_admin: user.is_admin,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct SlimUser {
    pub id: String,
    pub email: String,
    pub full_name: String,
    pub is_admin: bool,
}

impl From<&User> for SlimUser {
    fn from(user: &User) -> Self {
        SlimUser {
            id: user.id.clone().unwrap().to_string(),
            email: user.email.clone(),
            full_name: format!("{} {}", user.first_name, user.last_name),
            is_admin: user.is_admin,
        }
    }
}

pub fn unwrap_auth(
    auth_user: Option<ReqData<AuthenticatedUser>>,
) -> Result<AuthenticatedUser, AppError> {
    match auth_user {
        Some(user) => Ok(user.into_inner()),
        None => Err(AppError::Unauthorized),
    }
}

#[derive(Debug, Clone)]
pub enum UserFindableCol {
    #[allow(dead_code)]
    Username,

    #[allow(dead_code)]
    Email,
}

impl Into<String> for UserFindableCol {
    fn into(self) -> String {
        match self {
            Self::Username => "username".into(),
            Self::Email => "email".into(),
        }
    }
}
