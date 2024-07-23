use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Value, Datetime};



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JobListing {
    pub id: Option<Thing>,
    pub title: String,
    pub description: String,
    pub created_at: Datetime,
    pub author: Thing,
    pub tenant: Thing,
}

impl From<JobListing> for Value {
    fn from(job_listing: JobListing) -> Self {
        let mut job_listing_map = crate::data_map![
            "title" => job_listing.title.into(),
            "description" => job_listing.description.into(),
            "created_at" => job_listing.created_at.into(),
            "author" => job_listing.author.into(),
            "tenant" => job_listing.tenant.into(),
        ];

        if let Some(id) = job_listing.id {
            job_listing_map.insert("id".into(), id.into());
        }

        Value::from(job_listing_map)
    }
}
