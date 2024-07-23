use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Value};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Group {
    pub id: Option<Thing>,
    pub name: String,
    pub group_type: String,
    pub owner: Thing,
    pub tenant: Thing,
    pub members: Vec<Thing>,
}

impl From<Group> for Value {
    fn from(group: Group) -> Self {
        let mut group_map = crate::data_map![
            "name" => group.name.into(),
            "group_type" => group.group_type.into(),
            "owner" => group.owner.into(),
            "tenant" => group.tenant.into(),
            "members" => group.members.into(),
        ];

        if let Some(id) = group.id {
            group_map.insert("id".into(), id.into());
        }

        Value::from(group_map)
    }
}
