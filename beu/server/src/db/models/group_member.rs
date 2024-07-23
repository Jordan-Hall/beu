use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Value};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupMember {
    pub id: Option<Thing>,
    pub from: Thing,
    pub to: Thing,
}

impl From<GroupMember> for Value {
    fn from(group_member: GroupMember) -> Self {
        let mut group_member_map = crate::data_map![
            "from" => group_member.from.into(),
            "to" => group_member.to.into(),
        ];

        if let Some(id) = group_member.id {
            group_member_map.insert("id".into(), id.into());
        }

        Value::from(group_member_map)
    }
}
