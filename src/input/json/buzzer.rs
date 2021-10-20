use crate::json::Validate;
use rocket::http::Status;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Param {
    pub mode: u8,
    pub duration: u32,
}

impl Validate for Param {
    fn validate(&self) -> Result<(), Status> {
        Ok(())
    }
}