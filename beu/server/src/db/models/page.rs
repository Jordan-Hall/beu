use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Value};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Page {
    pub id: Option<Thing>,
    pub name: String,
    pub page_type: String,
    pub owner: Thing,
    pub tenant: Thing,
}

impl From<Page> for Value {
    fn from(page: Page) -> Self {
        let mut page_map = crate::data_map![
            "name" => page.name.into(),
            "page_type" => page.page_type.into(),
            "owner" => page.owner.into(),
            "tenant" => page.tenant.into(),
        ];

        if let Some(id) = page.id {
            page_map.insert("id".into(), id.into());
        }

        Value::from(page_map)
    }
}
