use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Value, Datetime};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Comment {
    pub id: Option<Thing>,
    pub content: String,
    pub created_at: Datetime,
    pub author: Thing,
    pub post: Option<Thing>,
    pub article: Option<Thing>,
    pub parent: Option<Thing>,
}

impl From<Comment> for Value {
    fn from(comment: Comment) -> Self {
        let mut comment_map = crate::data_map![
            "content" => comment.content.into(),
            "created_at" => comment.created_at.into(),
            "author" => comment.author.into(),
            "post" => comment.post.into(),
            "article" => comment.article.into(),
            "parent" => comment.parent.into(),
        ];

        if let Some(id) = comment.id {
            comment_map.insert("id".into(), id.into());
        }

        Value::from(comment_map)
    }
}
