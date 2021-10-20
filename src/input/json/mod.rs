pub mod empty;
pub mod info;
pub mod buzzer;
pub mod watchdog;
pub mod shutdown;
pub mod cmc;
pub mod usbhub;


use rocket::http::Status;

pub trait Validate {
    fn validate(&self) -> Result<(),Status>;
}