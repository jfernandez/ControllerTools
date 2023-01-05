use anyhow::Result;
use hidapi::HidApi;
use log::{debug, error};
use serde::{Deserialize, Serialize};

const DS_VENDOR_ID: u16 = 0x054c;
const DS_PRODUCT_ID: u16 = 0x0ce6;
const DS_INPUT_REPORT_BT: u8 = 0x31;
const DS_INPUT_REPORT_BT_SIZE: usize = 78;
const DS_INPUT_REPORT_USB: u8 = 0x01;
const DS_INPUT_REPORT_USB_SIZE: usize = 64;
const DS_STATUS_BATTERY_CAPACITY: u8 = 0xF;
const DS_STATUS_CHARGING: u8 = 0xF0;
const DS_STATUS_CHARGING_SHIFT: u8 = 4;

pub struct API;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Controller {
    pub name: String,
    pub product_id: u16,
    pub vendor_id: u16,
    pub capacity: u8,
    pub status: String,
}

impl API {
    pub fn get_controllers(&self) -> Result<Vec<Controller>> {
        let hidapi = HidApi::new()?;
        let mut controllers: Vec<Controller> = Vec::new();

        for device_info in hidapi.device_list() {
            match (device_info.vendor_id(), device_info.product_id()) {
                (DS_VENDOR_ID, DS_PRODUCT_ID) => {
                    debug!("Found DualSense controller: {:?}", device_info);
                    let bluetooth = device_info.interface_number() == -1;
                    let device = hidapi.open(device_info.vendor_id(), device_info.product_id())?;

                    // Read data from device_info
                    let mut buf = [0u8; DS_INPUT_REPORT_BT_SIZE];
                    let res = device.read(&mut buf[..])?;
                    let mut controller = Controller {
                        name: "PlayStation DualSense".to_string(),
                        product_id: device_info.product_id(),
                        vendor_id: device_info.vendor_id(),
                        capacity: 0,
                        status: "unknown".to_string(),
                    };

                    if !bluetooth
                        && buf[0] == DS_INPUT_REPORT_USB
                        && res == DS_INPUT_REPORT_USB_SIZE
                    {
                        let foo = &buf[1..];
                        let ds_report: DualsenseInputReport = bincode::deserialize(foo)?;
                        let battery_data: u8 = ds_report.status & DS_STATUS_BATTERY_CAPACITY;
                        let charging_status: u8 =
                            (ds_report.status & DS_STATUS_CHARGING) >> DS_STATUS_CHARGING_SHIFT;
                        let battery_status = get_battery_status(charging_status, battery_data);
                        controller.capacity = battery_status.capacity;
                        controller.status = battery_status.status;
                    } else if bluetooth
                        && buf[0] == DS_INPUT_REPORT_BT
                        && res == DS_INPUT_REPORT_BT_SIZE
                    {
                        let ds_report: DualsenseInputReport = bincode::deserialize(&buf[1..])?;
                        let battery_data: u8 = ds_report.status & DS_STATUS_BATTERY_CAPACITY;
                        let charging_status: u8 =
                            (ds_report.status & DS_STATUS_CHARGING) >> DS_STATUS_CHARGING_SHIFT;
                        let battery_status = get_battery_status(charging_status, battery_data);
                        controller.capacity = battery_status.capacity;
                        controller.status = battery_status.status;
                    } else {
                        error!("Unhandled report ID: {}", buf[0]);
                    }

                    controllers.push(controller);
                }
                _ => {}
            }
        }
        return Ok(controllers);
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Deserialize)]
struct DualsenseTouchPoint {
    contact: u8,
    x_lo: u8,
    x_hi: u8,
    y_lo: u8,
    y_hi: u8,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Deserialize)]
struct DualsenseInputReport {
    x: u8,
    y: u8,
    rx: u8,
    ry: u8,
    z: u8,
    rz: u8,
    seq_number: u8,
    buttons: [u8; 4],
    reserved: [u8; 4],
    gyro: [u16; 3],
    accel: [u16; 3],
    sensor_timestamp: u32,
    reserved2: u8,
    points: [DualsenseTouchPoint; 2],
    reserved3: [u8; 12],
    status: u8,
    reserved4: [u8; 10],
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BatteryInfo {
    capacity: u8,
    status: String,
}

fn get_battery_status(charging_status: u8, battery_data: u8) -> BatteryInfo {
    fn min(a: u8, b: u8) -> u8 {
        if a < b {
            a
        } else {
            b
        }
    }

    match charging_status {
        0x0 => BatteryInfo {
            capacity: min(battery_data * 10 + 5, 100),
            status: "discharging".to_string(),
        },
        0x1 => BatteryInfo {
            capacity: min(battery_data * 10 + 5, 100),
            status: "charging".to_string(),
        },
        0x2 => BatteryInfo {
            capacity: 100,
            status: "full".to_string(),
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
