use std::io::BufRead;
use std::{fs::File, io, path::Path, process::Command};

use anyhow::Result;
use hidapi::{DeviceInfo, HidApi};
use log::error;
// use serde::{Deserialize, Serialize};

use super::Controller;

pub const MS_VENDOR_ID: u16 = 0x045e;
pub const MS_VENDOR_ID_STR: &str = "045e";

// Xbox One S controller
pub const XBOX_CONTROLLER_USB_PRODUCT_ID: u16 = 0x02ea; // 746
pub const XBOX_CONTROLLER_USB_PRODUCT_ID_STR: &str = "02ea"; // 746
pub const XBOX_CONTROLLER_PRODUCT_ID: u16 = 0x02df; // 765

// after upgrade to the latest firmware (same as Series X/S),
// the One S controller changed product ID!
pub const XBOX_ONE_S_LATEST_FW_PRODUCT_ID: u16 = 0x0b20; // 2848

// Xbox Wireless Controller (model 1914)
pub const XBOX_WIRELESS_CONTROLLER_USB_PRODUCT_ID: u16 = 0x0b12; // 2834
pub const XBOX_WIRELESS_CONTROLLER_USB_PRODUCT_ID_STR: &str = "0b12"; // 2834
pub const XBOX_WIRELESS_CONTROLLER_BT_PRODUCT_ID: u16 = 0x0b13; // 2835

// pub const XBOX_ONE_REPORT_BT_SIZE: usize = 64;

pub fn get_xbox_controller(product_id: u16, bluetooth: bool) -> Result<Controller> {
    let controller = Controller {
        name: if product_id == XBOX_WIRELESS_CONTROLLER_USB_PRODUCT_ID
            || product_id == XBOX_WIRELESS_CONTROLLER_BT_PRODUCT_ID
        {
            "Xbox Series X/S".to_string()
        } else {
            "Xbox One S".to_string()
        },
        product_id,
        vendor_id: MS_VENDOR_ID,
        capacity: if bluetooth { 0 } else { 100 }, // for now for USB, "fake" it and set capacity to 100 as charging
        status: if bluetooth {
            "unknown".to_string()
        } else {
            // for now for USB, "fake" it and set status to charging since it's plugged in
            "charging".to_string()
        },
        bluetooth,
    };

    Ok(controller)
}

/// Get the bluetooth address from the DeviceInfo's hidraw,
/// e.g. "/sys/class/hidraw/hidraw5/device/uevent".
/// This file contains the BT address as value of HID_UNIQ
fn get_bluetooth_address(device_info: &DeviceInfo) -> Result<String> {
    let mut bt_address = "".to_string();
    let hidraw_path = device_info.path().to_str()?;
    let prefix = hidraw_path.replace("/dev", "/sys/class/hidraw");
    let path = [prefix, "device/uevent".to_string()].join("/");
    let lines = read_lines(path)?;
    for line in lines {
        let val = line?;
        // HID_UNIQ points to the BT address we want to use to grab data from bluetoothctl
        if val.starts_with("HID_UNIQ") {
            match val.split("=").skip(1).next() {
                Some(address) => {
                    bt_address = address.to_string();
                }
                None => {}
            }
        }
    }
    Ok(bt_address.to_string())
}

/// For Xbox controllers, "bluetoothctl info <address>" will return info about the controller
/// including its battery percentage. This important output is:
/// "Battery Percentage: 0x42 (66)"
fn get_battery_percentage(address: String) -> Result<u8> {
    let mut percentage = 0;
    let output = Command::new("bluetoothctl")
        .args(["info", address.as_str()])
        .output()?;
    let content = String::from_utf8_lossy(&output.stdout).to_string();
    for bt_line in content.lines() {
        if bt_line.contains("Battery Percentage") {
            // format is: "Battery Percentage: 0x42 (66)"
            match bt_line.split(" ").skip(2).next() {
                Some(percentage_hex) => {
                    if let Ok(pct) = i64::from_str_radix(&percentage_hex[2..], 16) {
                        percentage = pct as u8;
                    }
                }
                None => {}
            }
        }
    }
    Ok(percentage)
}

pub fn parse_xbox_controller_data(
    device_info: &DeviceInfo,
    _hidapi: &HidApi,
) -> Result<Controller> {
    let bluetooth = device_info.interface_number() == -1;
    // let device = device_info.open_device(hidapi)?;

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
        capacity,
        status: if capacity > 0 {
            "discharging".to_string()
        } else {
            "unknown".to_string()
        },
        bluetooth,
    };

    Ok(controller)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
