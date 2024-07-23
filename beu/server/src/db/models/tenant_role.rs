use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Value};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TenantRole {
    pub id: Option<Thing>,
    pub from: Thing,
    pub to: Thing,
}

impl From<TenantRole> for Value {
    fn from(tenant_role: TenantRole) -> Self {
        let mut tenant_role_map = crate::data_map![
            "from" => tenant_role.from.into(),
            "to" => tenant_role.to.into(),
        ];

        if let Some(id) = tenant_role.id {
            tenant_role_map.insert("id".into(), id.into());
        }

        Value::from(tenant_role_map)
    }
}
