use rocket::{ State};

use crate::api::helper::Helper;
use crate::api_version::ApiVersion;
use crate::input::*;
use crate::response;
use crate::settings::Settings;
use rocket::serde::json::Json;
use nexus_unity_sdbp::sdbp::response::custom::bmc::cmc::*;
use crate::response::CResponse;
use nexus_unity_sdbp::sdbp::request::custom::bmc::CustomBuilderBmc;

#[post("/bmc/<version>/<slot>/usb_bootloader", data = "<param>")]
pub fn usb_bootloader(settings: &State<Settings>, version: ApiVersion, slot: u16, param: Json<json::cmc::Param>) -> CResponse {
    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &param) {
        Ok(value) => value,
        Err(err) => return err   };

    let command = match CustomBuilderBmc::cmc().set_usb_bootloader(param.enabled, param.timeout) {
        Ok(value) => value,
        Err(err) => return response::unprocessable_entity(err.to_string()),
    };

    let result: Result<Cmc, std::io::Error> = com_manager.device_command(command);
    return match result {
        Ok(value) => response::ok(serde_json::to_string_pretty(&value).unwrap()),
        Err(err) => {
            response::internal_server_error(format!("{}{}", "Internal driver communication failed: ".to_string(), err.to_string()))
        }
    }
}

#[post("/bmc/<version>/<slot>/hard_reset", data = "<param>")]
pub fn hard_reset(settings: &State<Settings>, version: ApiVersion, slot: u16, param: Json<json::cmc::ParamReset>) -> CResponse {
    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &param) {
        Ok(value) => value,
        Err(err) => return err };

    if param.reset != "yes" {
        return response::unprocessable_entity("value must be 'yes'".to_string());
    }

    let command = match CustomBuilderBmc::cmc().hard_reset() {
        Ok(value) => value,
        Err(err) => return response::unprocessable_entity(err.to_string()),
    };

    let result: Result<Reset, std::io::Error> = com_manager.device_command(command);
    return match result {
        Ok(value) => response::ok(serde_json::to_string_pretty(&value).unwrap()),
        Err(err) => {
            response::internal_server_error(format!("{}{}", "Internal driver communication failed: ".to_string(), err.to_string()))
        }
    }
}