use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Value, Datetime};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    pub id: Option<Thing>,
    pub user: Thing,
    pub tenant: Thing,
    pub bio: Option<String>,
    pub interests: Option<Vec<String>>,
    pub created_at: Datetime,
}

impl From<Profile> for Value {
    fn from(profile: Profile) -> Self {
        let mut profile_map = crate::data_map![
            "user" => profile.user.into(),
            "tenant" => profile.tenant.into(),
            "bio" => profile.bio.into(),
            "interests" => profile.interests.into(),
            "created_at" => profile.created_at.into(),
        ];

        if let Some(id) = profile.id {
            profile_map.insert("id".into(), id.into());
        }

        Value::from(profile_map)
    }
}
