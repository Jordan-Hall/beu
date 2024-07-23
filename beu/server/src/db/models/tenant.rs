use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Value};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tenant {
    pub id: Option<Thing>,
    pub name: String,
    pub tenant_type: String,
    pub user_id: Thing,
}

impl From<Tenant> for Value {
    fn from(tenant: Tenant) -> Self {
        let mut tenant_map = crate::data_map![
            "name" => tenant.name.into(),
            "tenant_type" => tenant.tenant_type.into(),
            "user_id" => tenant.user_id.into(),
        ];

        if let Some(id) = tenant.id {
            tenant_map.insert("id".into(), id.into());
        }

        Value::from(tenant_map)
    }
}
