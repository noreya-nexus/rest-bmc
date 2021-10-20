use nexus_unity_sdbp::sdbp::response::custom::bmc::usbhub::*;
use rocket::{State};
use rocket::serde::json::Json;

use crate::api::helper::Helper;
use crate::api_version::ApiVersion;
use crate::input::*;
use crate::response;
use crate::settings::Settings;
use crate::response::CResponse;
use nexus_unity_sdbp::sdbp::request::custom::bmc::CustomBuilderBmc;

#[get("/bmc/<version>/<slot>/usbhub")]
pub fn usbhub(settings: &State<Settings>, version: ApiVersion, slot: u16) -> CResponse {
    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &json::empty::Param::empty_json()) {
        Ok(value) => value,
        Err(err) => return err
    };

    let command = CustomBuilderBmc::usbhub().get_hub_state().unwrap();
    let result: Result<UsbHub, std::io::Error> = com_manager.device_command(command);
    let usb_hub_state = match result {
        Ok(value) => value,
        Err(err) => {
            warn!("{}", err);
            return response::internal_server_error("Failed getting usb hub state".to_string());
        }
    };

    let command = CustomBuilderBmc::usbhub().get_port_state().unwrap();
    let result: Result<UsbHubPort, std::io::Error> = com_manager.device_command(command);
    let hub_port_state = match result {
        Ok(value) => value,
        Err(err) => {
            warn!("{}", err);
            return response::internal_server_error("Failed getting usb hub port state".to_string());
        }
    };

    if map_state(usb_hub_state.state) == "not available" ||
        map_state(hub_port_state.port1) == "not available" ||
        map_state(hub_port_state.port2) == "not available" ||
        map_state(hub_port_state.port3) == "not available" ||
        map_state(hub_port_state.port4) == "not available" ||
        map_state(hub_port_state.port5) == "not available" ||
        map_state(hub_port_state.port6) == "not available" ||
        map_state(hub_port_state.port7) != "not available"
    {
        return response::internal_server_error("Failed converting usb hub port state".to_string());
    }

    let result = UsbHubRest {
        hub_state: map_state(usb_hub_state.state),
        port_slot_2: map_state(hub_port_state.port1),
        port_slot_3: map_state(hub_port_state.port2),
        port_slot_4: map_state(hub_port_state.port3),
        port_slot_5: map_state(hub_port_state.port4),
        port_slot_6: map_state(hub_port_state.port5),
        port_slot_7: map_state(hub_port_state.port6),
        port_slot_8: map_state(hub_port_state.port7),
    };

    response::ok(serde_json::to_string_pretty(&result).unwrap())
}

#[post("/bmc/<version>/<slot>/usbhub/port/<number>", data = "<param>")]
pub fn usbhub_set_port(settings: &State<Settings>, version: ApiVersion, slot: u16, number: u8, param: Json<json::usbhub::Param>) -> CResponse {
    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &param) {
        Ok(value) => value,
        Err(err) => return err
    };

    if number < 2 || number > 8 {
        return response::bad_request("Port is out of range".to_string());
    }
    let number = number - 1; // Offset between port and slot

    let command = CustomBuilderBmc::usbhub().set_port_state(param.state, number).unwrap();
    let result: Result<SetPortSuccess, std::io::Error> = com_manager.device_command(command);
    match result {
        Ok(value) => response::ok(serde_json::to_string_pretty(&value).unwrap()),
        Err(err) => {
            warn!("{}", err);
            return response::internal_server_error("Failed getting usb hub state".to_string());
        }
    }
}

#[post("/bmc/<version>/<slot>/usbhub", data = "<param>")]
pub fn usbhub_set_hub(settings: &State<Settings>, version: ApiVersion, slot: u16, param: Json<json::usbhub::Param>) -> CResponse {
    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &param) {
        Ok(value) => value,
        Err(err) => return err
    };

    let command = CustomBuilderBmc::usbhub().set_hub_state(param.state).unwrap();
    let result: Result<SetHubSuccess, std::io::Error> = com_manager.device_command(command);
    match result {
        Ok(value) => response::ok(serde_json::to_string_pretty(&value).unwrap()),
        Err(err) => {
            warn!("{}", err);
            return response::internal_server_error("Failed setting usb hub state".to_string());
        }
    }
}

#[post("/bmc/<version>/<slot>/usbhub/reset", data = "<param>")]
pub fn usbhub_reset(settings: &State<Settings>, version: ApiVersion, slot: u16, param: Json<json::usbhub::Reset>) -> CResponse {
    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &param) {
        Ok(value) => value,
        Err(err) => return err
    };

    if param.reset != "yes" {
        return response::unprocessable_entity("value must be 'yes'".to_string());
    }

    let command = CustomBuilderBmc::usbhub().hub_reset().unwrap();
    let result: Result<ResetSuccess, std::io::Error> = com_manager.device_command(command);
    match result {
        Ok(value) => response::ok(serde_json::to_string_pretty(&value).unwrap()),
        Err(err) => {
            warn!("{}", err);
            return response::internal_server_error("Failed setting usb hub state".to_string());
        }
    }
}
