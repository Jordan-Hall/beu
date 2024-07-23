use async_trait::async_trait;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::{Thing, Value, Uuid};
use surrealdb::Surreal;
use std::sync::Arc;
use std::collections::BTreeMap;

use crate::db::models::tenant::Tenant;
use crate::db::models::role::Role;
use crate::db::models::permission::Permission;
use crate::db::models::user::User;
use crate::db::models::audit_log::AuditLog;
use crate::db::models::notification::Notification;
use crate::db::models::file_upload::FileUpload;
use crate::utils::try_from::W;
use crate::app_error::AppError;
use crate::utils::password::password;

pub struct DB;

pub struct ConnectionOptions<'a> {
    pub namespace: &'a str,
    pub database: &'a str,
    pub credentials: Root<'a>,
}

impl DB {
    pub async fn connect<'a>(
        endpoint: &'static str,
        options: &ConnectionOptions<'a>,
    ) -> Result<Surreal<Client>, AppError> {
        let db = Surreal::new::<Ws>(endpoint).await?;
        db.signin(options.credentials).await?;
        db.use_ns(options.namespace).use_db(options.database).await?;
        Ok(db)
    }
}

#[derive(Clone)]
pub struct SurrealDBRepo {
    pub db: Arc<Surreal<Client>>,
}

impl SurrealDBRepo {
    pub async fn new(endpoint: &'static str, options: &ConnectionOptions<'_>) -> Self {
        let db = DB::connect(endpoint, options).await.unwrap();
        SurrealDBRepo {
            db: Arc::new(db),
        }
    }
}

#[async_trait]
pub trait DatabaseActions {
    async fn create_role(&self, name: &str) -> Result<Role, AppError>;
    async fn create_permission(&self, name: &str) -> Result<Permission, AppError>;
    async fn create_tenant(&self, name: &str, tenant_type: &str, user_id: &Thing) -> Result<Tenant, AppError>;
    async fn create_user(&self, name: &str, first_name: &str, last_name: &str, email: &str, password: &str) -> Result<User, AppError>;
    async fn get_tenant_data(&self, tenant_id: &Thing) -> Result<BTreeMap<String, Value>, AppError>;
    async fn log_action(&self, action: &str, user_id: &Thing, details: &Value) -> Result<AuditLog, AppError>;
    async fn request_password_reset(&self, email: &str) -> Result<Value, AppError>;
    async fn reset_password(&self, token: &str, new_password: &str) -> Result<Value, AppError>;
    async fn send_notification(&self, user_id: &Thing, message: &str) -> Result<Notification, AppError>;
    async fn get_notifications(&self, user_id: &Thing) -> Result<Vec<Notification>, AppError>;
    async fn upload_file(&self, user_id: &Thing, tenant_id: &Thing, file_path: &str, file_type: &str) -> Result<FileUpload, AppError>;
}

#[async_trait]
impl DatabaseActions for SurrealDBRepo {
    async fn create_role(&self, name: &str) -> Result<Role, AppError> {
        let role_query = format!("CREATE role SET name = '{}'", name);
        let role: Value = self.db.query(role_query).await?.take(0)?;

        let tenants: Vec<Thing> = self.db.query("SELECT * FROM tenant").await?.take(0)?;
        for tenant in tenants {
            let relation_query = format!("RELATE role:{}->tenant_role->tenant:{}", name, tenant.id);
            self.db.query(relation_query).await?;
        }

        Ok(W(role).try_into()?)
    }

    async fn create_permission(&self, name: &str) -> Result<Permission, AppError> {
        let permission_query = format!("CREATE permission SET name = '{}'", name);
        let permission: Value = self.db.query(permission_query).await?.take(0)?;

        let roles: Vec<Thing> = self.db.query("SELECT * FROM role").await?.take(0)?;
        for role in roles {
            let relation_query = format!("RELATE role:{}->role_permission->permission:{}", role.id, name);
            self.db.query(relation_query).await?;
        }

        Ok(W(permission).try_into()?)
    }

    async fn create_tenant(&self, name: &str, tenant_type: &str, user_id: &Thing) -> Result<Tenant, AppError> {
        todo!()
        // let tenant_query = format!("CREATE tenant SET name = '{}', type = '{}', user_id = '{}'", name, tenant_type, user_id.id);
        // let tenant: Value = self.db.query(tenant_query).await?.take(0)?;

        // let roles: Vec<Thing> = self.db.query("SELECT * FROM role").await?.take(0)?;
        // for role in roles {
        //     let relation_query = format!("RELATE role:{}->tenant_role->tenant:{}", role.id, tenant.);
        //     self.db.query(relation_query).await?;
        // }

        // Ok(W(tenant).try_into()?)
    }

    async fn create_user(&self, name: &str, first_name: &str, last_name: &str, email: &str, password: &str) -> Result<User, AppError> {
        todo!()
        // let hashed_password = password::hash_password(password.as_bytes()).unwrap();
        // let user_query = format!("CREATE user SET name = '{}', first_name = '{}', last_name = '{}', email = '{}', password = '{}'", name, first_name, last_name, email, hashed_password);
        // let user: Value = self.db.query(user_query).await?.take(0)?;

        // let user_thing = Thing::from(user.get("id").unwrap().to_string());
        // let personal_tenant = self.create_tenant(&format!("{} Personal", name), "personal", &user_thing).await?;
        // let professional_tenant = self.create_tenant(&format!("{} Professional", name), "professional", &user_thing).await?;

        // Ok(W(user).try_into()?)
    }

    async fn get_tenant_data(&self, tenant_id: &Thing) -> Result<BTreeMap<String, Value>, AppError> {
        let query = format!(
            "SELECT * FROM post WHERE tenant = '{}'; SELECT * FROM article WHERE tenant = '{}'; SELECT * FROM user WHERE id IN (SELECT from FROM tenant_member WHERE to = '{}')",
            tenant_id.id, tenant_id.id, tenant_id.id
        );
        let data: Value = self.db.query(query).await?.take(0)?;
        Ok(W(data).try_into()?)
    }

    async fn log_action(&self, action: &str, user_id: &Thing, details: &Value) -> Result<AuditLog, AppError> {
        let log_query = format!("CREATE audit_log SET action = '{}', user = '{}', details = '{}'", action, user_id.id, details.to_string());
        let log: Value = self.db.query(log_query).await?.take(0)?;
        Ok(W(log).try_into()?)
    }

    async fn request_password_reset(&self, email: &str) -> Result<Value, AppError> {
        let user_query = format!("SELECT * FROM user WHERE email = '{}'", email);
        let user: Option<Thing> = self.db.query(user_query).await?.take(0)?;
        if user.is_none() {
            return Err(AppError::BadRequest("User not found".into()));
        }

        let token = Uuid::new_v4().to_string();
        let reset_query = format!("CREATE password_reset_token SET user = '{}', token = '{}', created_at = time::now()", user.unwrap().id, token);
        let reset: Value = self.db.query(reset_query).await?.take(0)?;
        Ok(reset)
    }

    async fn reset_password(&self, token: &str, new_password: &str) -> Result<Value, AppError> {
        let reset_query = format!("SELECT * FROM password_reset_token WHERE token = '{}' AND created_at > time::now() - 24h", token);
        let reset: Option<Thing> = self.db.query(reset_query).await?.take(0)?;
        if reset.is_none() {
            return Err(AppError::BadRequest("Invalid or expired token".into()));
        }

        let hashed_password = password::hash_password(new_password).unwrap();
        let update_query = format!("UPDATE user SET password = '{}' WHERE id = '{}'", hashed_password, reset.unwrap().id);
        self.db.query(update_query).await?;
        let delete_query = format!("DELETE FROM password_reset_token WHERE token = '{}'", token);
        self.db.query(delete_query).await?;
        Ok(Value::Null)
    }

    async fn send_notification(&self, user_id: &Thing, message: &str) -> Result<Notification, AppError> {
        let notification_query = format!("CREATE notification SET user = '{}', message = '{}', created_at = time::now()", user_id.id, message);
        let notification: Value = self.db.query(notification_query).await?.take(0)?;
        Ok(W(notification).try_into()?)
    }

    async fn get_notifications(&self, user_id: &Thing) -> Result<Vec<Notification>, AppError> {
        let notifications_query = format!("SELECT * FROM notification WHERE user = '{}' ORDER BY created_at DESC", user_id.id);
        let notifications: Value = self.db.query(notifications_query).await?.take(0)?;
        Ok(W(notifications).try_into()?)
    }

    async fn upload_file(&self, user_id: &Thing, tenant_id: &Thing, file_path: &str, file_type: &str) -> Result<FileUpload, AppError> {
        let file_query = format!("CREATE file_upload SET user = '{}', tenant = '{}', file_path = '{}', file_type = '{}', created_at = time::now()", user_id.id, tenant_id.id, file_path, file_type);
        let file: Value = self.db.query(file_query).await?.take(0)?;
        Ok(W(file).try_into()?)
    }
}
