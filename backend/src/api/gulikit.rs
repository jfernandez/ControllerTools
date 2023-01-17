use anyhow::Result;
use hidapi::{DeviceInfo, HidApi};

use super::Controller;

pub const VENDOR_ID: u16 = 0x045e;
pub const PRODUCT_ID: u16 = 0x02e0;

pub fn parse_controller_data(device_info: &DeviceInfo, _hidapi: &HidApi) -> Result<Controller> {
    let bluetooth = device_info.interface_number() == -1;

    let controller = Controller {
        name: "GuliKit Controller XW".to_string(),
        product_id: device_info.product_id(),
        vendor_id: device_info.vendor_id(),
        capacity: 0,
        status: "unknown".to_string(),
        bluetooth,
        hid_device_path: device_info.path().to_str()?.to_string(),
    };

    Ok(controller)
}
