use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Value};



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TenantMember {
    pub id: Option<Thing>,
    pub from: Thing,
    pub to: Thing,
}

impl From<TenantMember> for Value {
    fn from(tenant_member: TenantMember) -> Self {
        let mut tenant_member_map = crate::data_map![
            "from" => tenant_member.from.into(),
            "to" => tenant_member.to.into(),
        ];

        if let Some(id) = tenant_member.id {
            tenant_member_map.insert("id".into(), id.into());
        }

        Value::from(tenant_member_map)
    }
}
