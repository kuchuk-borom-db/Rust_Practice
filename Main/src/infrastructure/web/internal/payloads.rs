use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VisLogEntry {
    pub operation_id: String,
    pub name: String,
    pub log_type: String,
    pub value: Option<String>,
    pub sequence: u32,
}
