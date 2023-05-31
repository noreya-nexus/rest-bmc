use noreya_sdbp::sdbp::response::custom::bmc::usbhub::*;
use rocket::{State};
use rocket::serde::json::Json;

use crate::api::helper::Helper;
use crate::api_version::ApiVersion;
use crate::input::*;
use crate::response;
use crate::settings::Settings;
use crate::response::CResponse;
use noreya_sdbp::sdbp::request::custom::bmc::CustomBuilderBmc;

fn get_usbhub(settings: &State<Settings>, version: ApiVersion, slot: u16) -> Result<UsbHubRest, CResponse> {
    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &json::empty::Param::empty_json()) {
        Ok(value) => value,
        Err(err) => return Err(err)
    };

    let command = CustomBuilderBmc::usbhub().get_hub_state().unwrap();
    let result: Result<UsbHub, std::io::Error> = com_manager.device_command(command);
    let usb_hub_state = match result {
        Ok(value) => value,
        Err(err) => {
            warn!("{}", err);
            return Err(response::internal_server_error("Failed getting usb hub state".to_string()));
        }
    };

    let command = CustomBuilderBmc::usbhub().get_slot_state().unwrap();
    let result: Result<UsbHubPort, std::io::Error> = com_manager.device_command(command);
    let hub_port_state = match result {
        Ok(value) => value,
        Err(err) => {
            warn!("{}", err);
            return Err(response::internal_server_error("Failed getting usb hub port state".to_string()));
        }
    };
    let test = vec![usb_hub_state.state,
                    hub_port_state.slot0,
                    hub_port_state.slot1,
                    hub_port_state.slot2,
                    hub_port_state.slot3,
                    hub_port_state.slot4,
                    hub_port_state.slot5,
                    hub_port_state.slot6,
                    hub_port_state.slot7];

    let mut res = Vec::new();
    for slot in test {
        match map_state(slot) {
            Ok(state) => {
                res.push(state);
            }
            Err(_) => {
                return Err(response::internal_server_error("Failed converting usb hub port state".to_string()));
            }
        }
    }

    let result = UsbHubRest {
        hub_state: res.get(0).expect("This is checked before").clone(),
        port_slot_0: res.get(1).expect("This is checked before").clone(),
        port_slot_1: res.get(2).expect("This is checked before").clone(),
        port_slot_2: res.get(3).expect("This is checked before").clone(),
        port_slot_3: res.get(4).expect("This is checked before").clone(),
        port_slot_4: res.get(5).expect("This is checked before").clone(),
        port_slot_5: res.get(6).expect("This is checked before").clone(),
        port_slot_6: res.get(7).expect("This is checked before").clone(),
        port_slot_7: res.get(8).expect("This is checked before").clone(),
    };

    return Ok(result);
}

#[get("/bmc/<version>/<slot>/usbhub")]
pub fn usbhub(settings: &State<Settings>, version: ApiVersion, slot: u16) -> CResponse {
    let resp = get_usbhub(settings,version,slot);

    match resp {
        Ok(result) => { response::ok(serde_json::to_string_pretty(&result).unwrap()) }
        Err(err) => {err}
    }
}

#[post("/bmc/<version>/<slot>/usbhub/slot/<number>", data = "<param>")]
pub fn usbhub_set_port(settings: &State<Settings>, version: ApiVersion, slot: u16, number: u8, param: Json<json::usbhub::Param>) -> CResponse {
    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &param) {
        Ok(value) => value,
        Err(err) => return err
    };

    if number < 2 || number > 7 {
        return response::bad_request("Slot is out of range".to_string());
    }

    let command = CustomBuilderBmc::usbhub().set_slot_state(param.state, number).unwrap();
    let result: Result<SetPortSuccess, std::io::Error> = com_manager.device_command(command);
    match result {
        Ok(value) => response::ok(serde_json::to_string_pretty(&value).unwrap()),
        Err(err) => {
            warn!("{}", err);
            return response::internal_server_error("Failed setting usb hub state".to_string());
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

#[get("/bmc/<version>/<slot>/usbhub/devices")]
pub fn usbhub_devices(settings: &State<Settings>, version: ApiVersion, slot: u16) -> CResponse {
    let mut disabled = Vec::new();
    match get_usbhub(settings,version.clone(),slot) {
        Ok(hub) => {
            let mut i = 0;
            for element in vec![hub.port_slot_0, hub.port_slot_1, hub.port_slot_2, hub.port_slot_3, hub.port_slot_4,
                                hub.port_slot_5, hub.port_slot_6, hub.port_slot_7 ]{
                if element == "disabled" || element == "not available" {
                    disabled.push(UsbHubMapping{ slot_number: i, is_disabled: true });
                }
                i += 1;
            }
        }
        Err(err) => {
            return err;
        }
    }
    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &json::empty::Param::empty_json()) {
        Ok(value) => value,
        Err(_err) => {return response::internal_server_error("Failed getting usb hub mapping".to_string())}
    };

    let command = CustomBuilderBmc::usbhub().get_port_mapping().unwrap();
    let result: Result<UsbHubPortMapping, std::io::Error> = com_manager.device_command(command);
    let port_mapping = match result {
        Ok(value) => value,
        Err(err) => {
            warn!("{}", err);
            return response::internal_server_error("Failed getting usb hub mapping".to_string());
        }
    };
    let usb = get_usb_devices(disabled, port_mapping);

    match usb {
        Ok(devices) => {
            response::ok(serde_json::to_string_pretty(&devices).unwrap())
        }
        Err(err) => {
            error!("{}",err);
            return response::internal_server_error("Failed setting usb hub state".to_string());
        }
    }
}