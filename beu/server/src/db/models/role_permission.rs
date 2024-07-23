use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Value};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RolePermission {
    pub id: Option<Thing>,
    pub from: Thing,
    pub to: Thing,
}

impl From<RolePermission> for Value {
    fn from(role_permission: RolePermission) -> Self {
        let mut role_permission_map = crate::data_map![
            "from" => role_permission.from.into(),
            "to" => role_permission.to.into(),
        ];

        if let Some(id) = role_permission.id {
            role_permission_map.insert("id".into(), id.into());
        }

        Value::from(role_permission_map)
    }
}
