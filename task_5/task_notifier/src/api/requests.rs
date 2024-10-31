use serde::Deserialize;

use crate::domain::models::TypeNotification;

#[derive(Deserialize)]
pub struct Params {
    #[serde(rename = "type")]
    pub notification_type: TypeNotification,
}
