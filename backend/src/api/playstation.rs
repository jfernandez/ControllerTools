use std::cmp;

use anyhow::Result;
use hidapi::{DeviceInfo, HidApi};
use log::error;
use serde::{Deserialize, Serialize};

use super::Controller;

// PlayStation 5 DualSense controller
pub const DS_VENDOR_ID: u16 = 0x054c;
pub const DS_PRODUCT_ID: u16 = 0x0ce6;

const DS_INPUT_REPORT_BT: u8 = 0x31;
const DS_INPUT_REPORT_BT_SIZE: usize = 78;
const DS_INPUT_REPORT_USB: u8 = 0x01;
const DS_INPUT_REPORT_USB_SIZE: usize = 64;
const DS_STATUS_BATTERY_CAPACITY: u8 = 0b1111;
const DS_STATUS_CHARGING: u8 = 0b1111 << 4;
const DS_STATUS_CHARGING_SHIFT: u8 = 4;

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Deserialize)]
struct DualSenseTouchPoint {
    contact: u8,
    x_lo: u8,
    _bitfield_align_1: [u8; 0],
    _bitfield_1: BitfieldUnit<[u8; 1usize]>,
    y_hi: u8,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Deserialize)]
struct BitfieldUnit<Storage> {
    storage: Storage,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Deserialize)]
struct DualSenseInputReport {
    x: u8,
    y: u8,
    rx: u8,
    ry: u8,
    z: u8,
    rz: u8,
    seq_number: u8,
    buttons: [u8; 4usize],
    reserved: [u8; 4usize],
    gyro: [u16; 3usize],
    accel: [u16; 3usize],
    sensor_timestamp: u32,
    reserved2: u8,
    points: [DualSenseTouchPoint; 2usize],
    reserved3: [u8; 12usize],
    status: u8,
    reserved4: [u8; 10usize],
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BatteryInfo {
    capacity: u8,
    status: String,
}

pub fn parse_dualsense_controller_data(
    device_info: &DeviceInfo,
    hidapi: &HidApi,
) -> Result<Controller> {
    let bluetooth = device_info.interface_number() == -1;
    let device = hidapi.open(device_info.vendor_id(), device_info.product_id())?;

    // Read data from device_info
    let mut buf = [0u8; DS_INPUT_REPORT_BT_SIZE];
    let res = device.read(&mut buf[..])?;
    let mut controller = Controller {
        name: "DualSense".to_string(),
        product_id: device_info.product_id(),
        vendor_id: device_info.vendor_id(),
        capacity: 0,
        status: "unknown".to_string(),
        bluetooth,
    };

    let ds_report: DualSenseInputReport;
    if !bluetooth && buf[0] == DS_INPUT_REPORT_USB && res == DS_INPUT_REPORT_USB_SIZE {
        ds_report = bincode::deserialize(&buf[1..])?;
    } else if bluetooth && buf[0] == DS_INPUT_REPORT_BT && res == DS_INPUT_REPORT_BT_SIZE {
        ds_report = bincode::deserialize(&buf[2..])?;
    } else {
        error!("Unhandled report ID: {}", buf[0]);
        return Ok(controller);
    }

    let battery_data = ds_report.status & DS_STATUS_BATTERY_CAPACITY;
    let charging_status = (ds_report.status & DS_STATUS_CHARGING) >> DS_STATUS_CHARGING_SHIFT;
    let battery_status = get_battery_status(charging_status, battery_data);
    controller.capacity = battery_status.capacity;
    controller.status = battery_status.status;

    Ok(controller)
}

fn get_battery_status(charging_status: u8, battery_data: u8) -> BatteryInfo {
    match charging_status {
        0x0 => BatteryInfo {
            capacity: cmp::min(battery_data * 10 + 5, 100),
            status: "discharging".to_string(),
        },
        0x1 => BatteryInfo {
            capacity: cmp::min(battery_data * 10 + 5, 100),
            status: "charging".to_string(),
        },
        0x2 => BatteryInfo {
            capacity: cmp::min(battery_data * 10 + 5, 100),
            status: "charging".to_string(),
        },
        0xa | 0xb => BatteryInfo {
            capacity: 0,
            status: "not-charging".to_string(),
        },
        0xf | _ => BatteryInfo {
            capacity: 0,
            status: "unknown".to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::api::playstation::{DualSenseInputReport, DS_INPUT_REPORT_USB_SIZE};

    #[test]
    fn test_dualsense_input_report_struct_size() {
        // Common input report size equals the size of the USB report minus 1 byte for the ReportID
        assert_eq!(
            std::mem::size_of::<DualSenseInputReport>(),
            DS_INPUT_REPORT_USB_SIZE - 1
        );
    }
}
