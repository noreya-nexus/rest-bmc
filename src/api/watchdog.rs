use noreya_sdbp::sdbp::response::custom::bmc::watchdog::*;
use rocket::{State};
use rocket::serde::json::Json;

use crate::api::helper::Helper;
use crate::api_version::ApiVersion;
use crate::input::*;
use crate::response;
use crate::settings::Settings;
use crate::response::CResponse;
use noreya_sdbp::sdbp::request::custom::bmc::CustomBuilderBmc;

#[post("/bmc/<version>/<slot>/watchdog", data = "<param>")]
pub fn timeout(settings: &State<Settings>, version: ApiVersion, slot: u16, param: Json<json::watchdog::Param>) -> CResponse {
    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &param) {
        Ok(value) => value,
        Err(err) => return err
    };

    let command = match CustomBuilderBmc::watchdog().timeout(param.timeout) {
        Ok(value) => value,
        Err(err) => return response::unprocessable_entity(err.to_string()),
    };

    let result: Result<json_response::TimeoutResponse, std::io::Error> = com_manager.device_command(command);
    return match result {
        Ok(value) => response::ok(serde_json::to_string_pretty(&value).unwrap()),
        Err(err) => {
            response::internal_server_error(format!("{}{}", "Internal driver communication failed: ".to_string(), err.to_string()))
        }
    };
}

#[post("/bmc/<version>/<slot>/watchdog/sw_shutdown_timeout", data = "<param>")]
pub fn sw_shutdown_timeout(settings: &State<Settings>, version: ApiVersion, slot: u16, param: Json<json::watchdog::Param>) -> CResponse {
    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &param) {
        Ok(value) => value,
        Err(err) => return err
    };

    let command = match CustomBuilderBmc::watchdog().set_shutdown_timeout(param.timeout) {
        Ok(value) => value,
        Err(err) => return response::unprocessable_entity(err.to_string()),
    };

    let result: Result<json_response::SetShutdownTimeout, std::io::Error> = com_manager.device_command(command);
    return match result {
        Ok(value) => response::ok(serde_json::to_string_pretty(&value).unwrap()),
        Err(err) => {
            response::internal_server_error(format!("{}{}", "Internal driver communication failed: ".to_string(), err.to_string()))
        }
    };
}


#[get("/bmc/<version>/<slot>/watchdog")]
pub fn stats(settings: &State<Settings>, version: ApiVersion, slot: u16) -> CResponse {
    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &json::empty::Param::empty_json()) {
        Ok(value) => value,
        Err(err) => return err
    };

    let command = CustomBuilderBmc::watchdog().get_timeout().unwrap();
    let result: Result<ipc::GetTimeout, std::io::Error> = com_manager.device_command(command);
    let get_timeout = match result {
        Ok(value) => value,
        Err(err) => {
            return response::internal_server_error(format!("{}{}", "Internal driver communication failed: ".to_string(), err.to_string()));
        }
    };

    let command = CustomBuilderBmc::watchdog().get_timeout_left().unwrap();
    let result: Result<ipc::GetTimeout, std::io::Error> = com_manager.device_command(command);
    let get_timeout_left = match result {
        Ok(value) => value,
        Err(err) => {
            return response::internal_server_error(format!("{}{}", "Internal driver communication failed: ".to_string(), err.to_string()));
        }
    };

    let command = CustomBuilderBmc::watchdog().get_shutdown_timeout().unwrap();
    let result: Result<ipc::GetTimeout, std::io::Error> = com_manager.device_command(command);
    let get_shutdown_timeout = match result {
        Ok(value) => value,
        Err(err) => {
            return response::internal_server_error(format!("{}{}", "Internal driver communication failed: ".to_string(), err.to_string()));
        }
    };

    let command = CustomBuilderBmc::watchdog().get_emergency_mode_state().unwrap();
    let result: Result<ipc::GetEmergency, std::io::Error> = com_manager.device_command(command);
    let emergency_mode = match result {
        Ok(value) => value,
        Err(err) => {
            return response::internal_server_error(format!("{}{}", "Internal driver communication failed: ".to_string(), err.to_string()));
        }
    };

    let result = json_response::Timeout {
        timeout: get_timeout.timeout,
        timeout_left: get_timeout_left.timeout,
        shutdown_timeout: get_shutdown_timeout.timeout,
        emergency_mode: emergency_mode.status,
    };

    response::ok(serde_json::to_string_pretty(&result).unwrap())
}

#[get("/bmc/<version>/<slot>/watchdog/alive")]
pub fn alive(settings: &State<Settings>, version: ApiVersion, slot: u16) -> CResponse {
    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &json::empty::Param::empty_json()) {
        Ok(value) => value,
        Err(err) => return err
    };

    let command = CustomBuilderBmc::watchdog().alive().unwrap();
    let result: Result<json_response::AliveResponse, std::io::Error> = com_manager.device_command(command);

    return match result {
        Ok(value) => response::ok(serde_json::to_string_pretty(&value).unwrap()),
        Err(err) => {
            response::internal_server_error(format!("{}{}", "Internal driver communication failed: ".to_string(), err.to_string()))
        }
    };
}

#[get("/bmc/<version>/<slot>/watchdog/save")]
pub fn save(settings: &State<Settings>, version: ApiVersion, slot: u16) -> CResponse {
    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &json::empty::Param::empty_json()) {
        Ok(value) => value,
        Err(err) => return err
    };

    let command = CustomBuilderBmc::watchdog().save_config().unwrap();
    let result: Result<json_response::SaveConfig, std::io::Error> = com_manager.device_command(command);

    return match result {
        Ok(value) => response::ok(serde_json::to_string_pretty(&value).unwrap()),
        Err(err) => {
            response::internal_server_error(format!("{}{}", "Internal driver communication failed: ".to_string(), err.to_string()))
        }
    };
}

#[post("/bmc/<version>/<slot>/watchdog/shutdown", data = "<param>")]
pub fn shutdown(settings: &State<Settings>, version: ApiVersion, slot: u16, param: Json<json::shutdown::Param>) -> CResponse {
    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &param) {
        Ok(value) => value,
        Err(err) => return err
    };

    if param.shutdown != "start" {
        return response::bad_request("value must be 'start'".to_string());
    }

    let command = CustomBuilderBmc::watchdog().sw_shutdown().unwrap();
    let result: Result<json_response::SwShutdown, std::io::Error> = com_manager.device_command(command);

    return match result {
        Ok(value) => response::ok(serde_json::to_string_pretty(&value).unwrap()),
        Err(err) => {
            response::internal_server_error(format!("{}{}", "Internal driver communication failed: ".to_string(), err.to_string()))
        }
    };
}