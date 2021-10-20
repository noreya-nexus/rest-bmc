use crate::json::Validate;
use rocket::http::Status;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Param {
    pub state: bool
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Reset {
    pub reset: String
}

impl Validate for Param {
    fn validate(&self) -> Result<(), Status> {
        Ok(())
    }
}

impl Validate for Reset {
    fn validate(&self) -> Result<(), Status> {
        Ok(())
    }
}