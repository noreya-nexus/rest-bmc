use rocket::{State};

use crate::api::helper::Helper;
use crate::api_version::ApiVersion;
use crate::{response, SharedData};
use crate::settings::{Settings};

use crate::response::CResponse;
use noreya_sdbp::sdbp::CoreBuilder;
use noreya_sdbp::sdbp::response::core::control::{SuspendResponse, RunResponse};
use crate::input::json::reset::ResetJson;
use rocket::serde::json::Json;

#[post("/bmc/<version>/<slot>/reset", data="<param>")]
pub fn module_reset(settings: &State<Settings>, version: ApiVersion, shared: &State<SharedData>, slot: u16, param : Json<ResetJson>) -> CResponse {

    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &param) {
        Ok(value) => value,
        Err(err) => return err,
    };
    let command = CoreBuilder::new().control().mode_suspend();

    let result : Result<SuspendResponse,std::io::Error> = com_manager.device_command(command.expect("Internal error"));
    match result {
        Ok(_value) => (),
        Err(err) => return response::internal_server_error(err.to_string()),
    };

    let mut lock = shared.notifications.lock().expect("Could not lock mutex");
    if !lock.is_empty() {
        match lock.get_mut(&slot) {
            None => {
                return response::internal_server_error("Could not find notification".to_string());
            }
            Some(_notification) => {
                // Notification not implemented for this module
            }
        }
    }

    let command = CoreBuilder::new().control().mode_run();
    let result : Result<RunResponse,std::io::Error> = com_manager.device_command(command.expect("Internal error"));
    let response = match result {
        Ok(_value) => response::ok("{ \"status\": \"success\" }".to_string()),
        Err(err) => response::internal_server_error(err.to_string()),
    };

    return response;
}
