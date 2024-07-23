use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Value};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PageMember {
    pub id: Option<Thing>,
    pub from: Thing,
    pub to: Thing,
}

impl From<PageMember> for Value {
    fn from(page_member: PageMember) -> Self {
        let mut page_member_map = crate::data_map![
            "from" => page_member.from.into(),
            "to" => page_member.to.into(),
        ];

        if let Some(id) = page_member.id {
            page_member_map.insert("id".into(), id.into());
        }

        Value::from(page_member_map)
    }
}
