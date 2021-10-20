use nexus_unity_sdbp::sdbp::response::custom::bmc::buzzer::*;
use rocket::{State};

use crate::api::helper::Helper;
use crate::api_version::ApiVersion;
use crate::input::*;
use crate::response;
use crate::settings::Settings;
use rocket::serde::json::Json;
use crate::response::CResponse;
use nexus_unity_sdbp::sdbp::request::custom::bmc::CustomBuilderBmc;

#[post("/bmc/<version>/<slot>/buzzer", data = "<param>")]
pub fn buzzer(settings: &State<Settings>, version: ApiVersion, slot: u16, param: Json<json::buzzer::Param>) -> CResponse {
    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &param) {
        Ok(value) => value,
        Err(err) =>  return err };

    let command = match CustomBuilderBmc::buzzer().buzzer(param.mode, param.duration) {
        Ok(value) => value,
        Err(err) => return response::unprocessable_entity(err.to_string()),
    };

    let result: Result<Buzzer, std::io::Error> = com_manager.device_command(command);
    return match result {
        Ok(value) => response::ok(serde_json::to_string_pretty(&value).unwrap()),
        Err(err) => {
            response::internal_server_error(format!("{}{}", "Internal driver communication failed: ".to_string(), err.to_string()))
        }
    }
}
