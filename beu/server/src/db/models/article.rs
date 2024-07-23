use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Value, Datetime};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Article {
    pub id: Option<Thing>,
    pub title: String,
    pub content: String,
    pub created_at: Datetime,
    pub author: Thing,
    pub tenant: Thing,
}

impl From<Article> for Value {
    fn from(article: Article) -> Self {
        let mut article_map = crate::data_map![
            "title" => article.title.into(),
            "content" => article.content.into(),
            "created_at" => article.created_at.into(),
            "author" => article.author.into(),
            "tenant" => article.tenant.into(),
        ];

        if let Some(id) = article.id {
            article_map.insert("id".into(), id.into());
        }

        Value::from(article_map)
    }
}
