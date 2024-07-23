use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Value};



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Permission {
    pub id: Option<Thing>,
    pub name: String,
}

impl From<Permission> for Value {
    fn from(permission: Permission) -> Self {
        let mut permission_map = crate::data_map![
            "name" => permission.name.into(),
        ];

        if let Some(id) = permission.id {
            permission_map.insert("id".into(), id.into());
        }

        Value::from(permission_map)
    }
}
