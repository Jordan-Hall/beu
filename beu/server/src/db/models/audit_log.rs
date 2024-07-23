use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Value, Datetime};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuditLog {
    pub id: Option<Thing>,
    pub action: String,
    pub user: Thing,
    pub timestamp: Datetime,
    pub details: serde_json::Value,
}

impl From<AuditLog> for Value {
    fn from(audit_log: AuditLog) -> Self {
        let mut audit_log_map = crate::data_map![
            "action" => audit_log.action.into(),
            "user" => audit_log.user.into(),
            "timestamp" => audit_log.timestamp.into(),
            "details" => audit_log.details.into(),
        ];

        if let Some(id) = audit_log.id {
            audit_log_map.insert("id".into(), id.into());
        }

        Value::from(audit_log_map)
    }
}
