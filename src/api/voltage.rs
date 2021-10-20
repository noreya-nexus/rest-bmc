use nexus_unity_sdbp::sdbp::request::custom::bmc::protocol::classes as vclass;
use nexus_unity_sdbp::sdbp::response::custom::bmc::voltage::*;
use nexus_unity_sdbp::sdbp::request::custom::bmc::CustomBuilderBmc;
use rocket::{State};

use crate::api::helper::Helper;
use crate::api_version::ApiVersion;
use crate::input::*;
use crate::response;
use crate::settings::Settings;
use crate::response::CResponse;

#[get("/bmc/<version>/<slot>/voltage")]
pub fn voltage(settings: &State<Settings>, version: ApiVersion, slot: u16) -> CResponse {
    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &json::empty::Param::empty_json()) {
        Ok(value) => value,
        Err(err) => return err
    };

    let result_1v8: Result<Voltage, std::io::Error> = com_manager.device_command(CustomBuilderBmc::voltage()
        .input(vclass::input::operation_code::input::RAIL_1V8).unwrap());
    let result_1v8= match result_1v8 {
        Ok(value) => value.voltage,
        Err(err) => {
            return response::internal_server_error(format!("{}{}", "Internal driver communication failed: ".to_string(), err.to_string()))
        }
    };

    let result_rtc: Result<Voltage, std::io::Error> = com_manager.device_command(CustomBuilderBmc::voltage()
        .input(vclass::input::operation_code::input::RTC_BAT).unwrap());
    let result_rtc= match result_rtc {
        Ok(value) => value.voltage,
        Err(err) => {
            return response::internal_server_error(format!("{}{}", "Internal driver communication failed: ".to_string(), err.to_string()))
        }
    };

    let result = VoltageRest {
        voltage_1v8: result_1v8 / 1000, // Convert to mV
        voltage_rtc: result_rtc / 1000, // Convert to mV
    };

    response::ok(serde_json::to_string_pretty(&result).unwrap())
}

fn calc_temp(ntc_voltage_uv: u32) -> f32
{
    let ntc_volt: f32 = ntc_voltage_uv as f32 / 1_000_000.0; // Convert uV to V
    let res: f32 = (3.3 - ntc_volt) / (ntc_volt / 10000.0);

    let beta = 3380.0;
    let ro = 10000.0;
    let to = 25.0;

    let mut steinhard = (res / ro).ln() / beta;
    steinhard = steinhard + (1.0 / (to + 273.15));
    steinhard = (1.0 / steinhard) - 273.15;

    return steinhard;
}

#[get("/bmc/<version>/<slot>/temperature")]
pub fn temperature(settings: &State<Settings>, version: ApiVersion, slot: u16) -> CResponse {
    let mut com_manager = match Helper::init_api_device_command(&settings, version, slot, &json::empty::Param::empty_json()) {
        Ok(value) => value,
        Err(err) =>  return err
    };

    let result_ntc_0: Result<Voltage, std::io::Error> = com_manager.device_command(CustomBuilderBmc::voltage()
        .input(vclass::input::operation_code::input::NTC_0).unwrap());
    let result_ntc_1: Result<Voltage, std::io::Error> = com_manager.device_command(CustomBuilderBmc::voltage()
        .input(vclass::input::operation_code::input::NTC_1).unwrap());

    let result_ntc_0= match result_ntc_0 {
        Ok(value) => value.voltage,
        Err(err) => {
            return response::internal_server_error(format!("{}{}", "Internal driver communication failed: ".to_string(), err.to_string()))
        }
    };

    let result_ntc_1= match result_ntc_1 {
        Ok(value) => value.voltage,
        Err(err) => {
            return response::internal_server_error(format!("{}{}", "Internal driver communication failed: ".to_string(), err.to_string()))
        }
    };

    let ntc0 = calc_temp(result_ntc_0);
    let ntc1 = calc_temp(result_ntc_1);

    let result = TemperatureRest {
        ntc_0: ntc0,
        ntc_1: ntc1,
    };

    response::ok(serde_json::to_string_pretty(&result).unwrap())
}