use anyhow::Result;
use hidapi::{DeviceInfo, HidApi};
// use log::debug;
// use serde::{Deserialize, Serialize};

use super::Controller;

pub const MS_VENDOR_ID: u16 = 0x045e;

// Xbox One S controller
pub const XBOX_CONTROLLER_PRODUCT_ID: u16 = 765 as u16;

// Xbox Wireless Controller (model 1914)
pub const XBOX_WIRELESS_CONTROLLER_USB_PRODUCT_ID: u16 = 2834 as u16; // 0x0b12
pub const XBOX_WIRELESS_CONTROLLER_BT_PRODUCT_ID: u16 = 2835 as u16;

// pub const XBOX_ONE_REPORT_BT_SIZE: usize = 64;

pub fn parse_xbox_controller_data(
    device_info: &DeviceInfo,
    _hidapi: &HidApi,
) -> Result<Controller> {
    let bluetooth = device_info.interface_number() == -1;
    // let device = device_info.open_device(hidapi)?;

    // TODO Read data from device_info to maybe get battery data?
    // so far we couldn't figure out how
    // let mut buf = [0u8; XBOX_ONE_REPORT_BT_SIZE];
    // let res = device.read(&mut buf[..])?;
    let controller = Controller {
        name: if device_info.product_id() == XBOX_WIRELESS_CONTROLLER_USB_PRODUCT_ID
            || device_info.product_id() == XBOX_WIRELESS_CONTROLLER_BT_PRODUCT_ID
        {
            "Xbox Series X/S".to_string()
        } else {
            "Xbox One S".to_string()
        },
        product_id: device_info.product_id(),
        vendor_id: device_info.vendor_id(),
        capacity: 0,
        status: "unknown".to_string(),
        bluetooth,
    };

    Ok(controller)
}
