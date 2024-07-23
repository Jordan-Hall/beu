use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Value, Datetime};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Notification {
    pub id: Option<Thing>,
    pub user: Thing,
    pub message: String,
    pub is_read: bool,
    pub created_at: Datetime,
}

impl From<Notification> for Value {
    fn from(notification: Notification) -> Self {
        let mut notification_map = crate::data_map![
            "user" => notification.user.into(),
            "message" => notification.message.into(),
            "is_read" => notification.is_read.into(),
            "created_at" => notification.created_at.into(),
        ];

        if let Some(id) = notification.id {
            notification_map.insert("id".into(), id.into());
        }

        Value::from(notification_map)
    }
}
