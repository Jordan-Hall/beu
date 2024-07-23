use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Value};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemberRole {
    pub id: Option<Thing>,
    pub from: Thing,
    pub to: Thing,
}

impl From<MemberRole> for Value {
    fn from(member_role: MemberRole) -> Self {
        let mut member_role_map = crate::data_map![
            "from" => member_role.from.into(),
            "to" => member_role.to.into(),
        ];

        if let Some(id) = member_role.id {
            member_role_map.insert("id".into(), id.into());
        }

        Value::from(member_role_map)
    }
}
