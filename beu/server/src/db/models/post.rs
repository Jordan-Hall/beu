use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Value, Datetime};



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    pub id: Option<Thing>,
    pub content: String,
    pub created_at: Datetime,
    pub author: Thing,
    pub tenant: Thing,
}

impl From<Post> for Value {
    fn from(post: Post) -> Self {
        let mut post_map = crate::data_map![
            "content" => post.content.into(),
            "created_at" => post.created_at.into(),
            "author" => post.author.into(),
            "tenant" => post.tenant.into(),
        ];

        if let Some(id) = post.id {
            post_map.insert("id".into(), id.into());
        }

        Value::from(post_map)
    }
}
