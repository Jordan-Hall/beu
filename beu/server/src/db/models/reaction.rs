use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Value};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reaction {
    pub id: Option<Thing>,
    pub reaction_type: String,
    pub user: Thing,
    pub post: Option<Thing>,
    pub article: Option<Thing>,
    pub comment: Option<Thing>,
}

impl From<Reaction> for Value {
    fn from(reaction: Reaction) -> Self {
        let mut reaction_map = crate::data_map![
            "reaction_type" => reaction.reaction_type.into(),
            "user" => reaction.user.into(),
            "post" => reaction.post.into(),
            "article" => reaction.article.into(),
            "comment" => reaction.comment.into(),
        ];

        if let Some(id) = reaction.id {
            reaction_map.insert("id".into(), id.into());
        }

        Value::from(reaction_map)
    }
}
