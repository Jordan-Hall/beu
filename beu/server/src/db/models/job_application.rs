use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Value, Datetime};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JobApplication {
    pub id: Option<Thing>,
    pub job: Thing,
    pub applicant: Thing,
    pub resume: String,
    pub cover_letter: Option<String>,
    pub status: String,
    pub applied_at: Datetime,
}

impl From<JobApplication> for Value {
    fn from(job_application: JobApplication) -> Self {
        let mut job_application_map = crate::data_map![
            "job" => job_application.job.into(),
            "applicant" => job_application.applicant.into(),
            "resume" => job_application.resume.into(),
            "cover_letter" => job_application.cover_letter.into(),
            "status" => job_application.status.into(),
            "applied_at" => job_application.applied_at.into(),
        ];

        if let Some(id) = job_application.id {
            job_application_map.insert("id".into(), id.into());
        }

        Value::from(job_application_map)
    }
}
