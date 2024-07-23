use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Value, Datetime};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileUpload {
    pub id: Option<Thing>,
    pub user: Thing,
    pub tenant: Thing,
    pub file_path: String,
    pub file_type: String,
    pub created_at: Datetime,
}

impl From<FileUpload> for Value {
    fn from(file_upload: FileUpload) -> Self {
        let mut file_upload_map = crate::data_map![
            "user" => file_upload.user.into(),
            "tenant" => file_upload.tenant.into(),
            "file_path" => file_upload.file_path.into(),
            "file_type" => file_upload.file_type.into(),
            "created_at" => file_upload.created_at.into(),
        ];

        if let Some(id) = file_upload.id {
            file_upload_map.insert("id".into(), id.into());
        }

        Value::from(file_upload_map)
    }
}
