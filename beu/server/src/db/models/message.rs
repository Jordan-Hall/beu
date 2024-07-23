use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Value, Datetime};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub id: Option<Thing>,
    pub content: String,
    pub created_at: Datetime,
    pub sender: Thing,
    pub recipient: Thing,
    pub tenant: Thing,
}

impl From<Message> for Value {
    fn from(message: Message) -> Self {
        let mut message_map = crate::data_map![
            "content" => message.content.into(),
            "created_at" => message.created_at.into(),
            "sender" => message.sender.into(),
            "recipient" => message.recipient.into(),
            "tenant" => message.tenant.into(),
        ];

        if let Some(id) = message.id {
            message_map.insert("id".into(), id.into());
        }

        Value::from(message_map)
    }
}
