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

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct DualSenseTouchPoint {
    pub contact: u8,
    pub x_lo: u8,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: BitfieldUnit<[u8; 1usize]>,
    pub y_hi: u8,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct BitfieldUnit<Storage> {
    storage: Storage,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct DualSenseInputreport {
    pub x: u8,
    pub y: u8,
    pub rx: u8,
    pub ry: u8,
    pub z: u8,
    pub rz: u8,
    pub seq_number: u8,
    pub buttons: [u8; 4usize],
    pub reserved: [u8; 4usize],
    pub gyro: [u16; 3usize],
    pub accel: [u16; 3usize],
    pub sensor_timestamp: u32,
    pub reserved2: u8,
    pub points: [DualSenseTouchPoint; 2usize],
    pub reserved3: [u8; 12usize],
    pub status: u8,
    pub reserved4: [u8; 10usize],
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
                        let ds_report: DualSenseInputreport = bincode::deserialize(foo)?;
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
                        let ds_report: DualSenseInputreport = bincode::deserialize(&buf[2..])?;
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
