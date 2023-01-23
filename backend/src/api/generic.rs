use crate::controller::Controller;

use super::bluetooth::{get_battery_percentage, get_bluetooth_address};
use super::nintendo::VENDOR_ID_NINTENDO;
use super::playstation::DS_VENDOR_ID;
use super::xbox::MS_VENDOR_ID;

use anyhow::Result;
use hidapi::{DeviceInfo, HidApi};
use log::error;

const VALVE_VENDOR_ID: u16 = 0x28de;
const FOCALTECH_VENDOR_ID: u16 = 0x2808; // touchpad?
pub const IGNORED_VENDORS: [u16; 5] = [
    VALVE_VENDOR_ID,
    FOCALTECH_VENDOR_ID,
    VENDOR_ID_NINTENDO,
    DS_VENDOR_ID,
    MS_VENDOR_ID,
];

pub fn get_controller_data(device_info: &DeviceInfo, _hidapi: &HidApi) -> Result<Controller> {
    let capacity: u8 = match get_bluetooth_address(device_info) {
        Ok(address) => match get_battery_percentage(address) {
            Ok(percentage) => percentage,
            Err(err) => {
                error!("get_battery_percentage failed because {}", err);
                0
            }
        },
        Err(err) => {
            error!("get_bluetooth_address failed because {}", err);
            0
        }
    };

    let mut name = device_info.product_string().unwrap_or("Unknown Controller");
    if name.starts_with("Stadia") {
        // product string is e.g. Stadia-CG9S-4e9f, this would be better
        name = "Stadia Controller";
    }

    let controller = Controller::from_hidapi(device_info, name, capacity, "unknown");
    Ok(controller)
}
