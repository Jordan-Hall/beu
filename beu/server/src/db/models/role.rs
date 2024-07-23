use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Value};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Role {
    pub id: Option<Thing>,
    pub name: String,
}

impl From<Role> for Value {
    fn from(role: Role) -> Self {
        let mut role_map = crate::data_map![
            "name" => role.name.into(),
        ];

        if let Some(id) = role.id {
            role_map.insert("id".into(), id.into());
        }

        Value::from(role_map)
    }
}
