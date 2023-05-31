use crate::json::Validate;
use rocket::http::Status;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Param {
    pub timeout: u32 // IMPROVEMENT: Add unit suffix
}

impl Validate for Param {
    fn validate(&self) -> Result<(), Status> {
        Ok(())
    }
}