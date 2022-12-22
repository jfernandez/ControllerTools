use serde::{Deserialize, Serialize};
use usdpl_back::core::serdes::Primitive;
use log::error;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const NAME: &'static str = env!("CARGO_PKG_NAME");

const VENDOR_ID: u16 = 0x054c;
const PRODUCT_ID: u16 = 0x0ce6;
// const INPUT_REPORT_BT: u16 = 0x31;
const INPUT_REPORT_BT_SIZE: usize = 78;
// const INPUT_REPORT_USB: u16 = 0x02;
// const DS_OUTPUT_REPORT_USB_SIZE: u16 = 78;
const DS_STATUS_BATTERY_CAPACITY: u8 = 0xF;
const DS_STATUS_CHARGING: u8 = 0xF0;
const DS_STATUS_CHARGING_SHIFT: u8 = 4;

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

pub fn get_battery_status(_: Vec<Primitive>) -> Vec<Primitive> {
  let api = hidapi::HidApi::new().unwrap();

  for device in api.device_list() {
    if device.vendor_id() == VENDOR_ID && device.product_id() == PRODUCT_ID {
        println!("Found DualSense: {:#?}", device);
        let device = api.open(device.vendor_id(), device.product_id()).unwrap();
        // Read data from device
        let mut buf: [u8; INPUT_REPORT_BT_SIZE] = [0u8; INPUT_REPORT_BT_SIZE];
        let res = device.read(&mut buf[..]).unwrap();

        println!("res: {}", res);
        println!("Read: {:?}", &buf[..res]);

        let ds_report: DualsenseInputReport = bincode::deserialize(&buf).unwrap();
        println!("DualsenseInputReport: {:#?}", ds_report);
        let battery_data: u8 = ds_report.status & DS_STATUS_BATTERY_CAPACITY;
        let charging_status: u8 =
            (ds_report.status & DS_STATUS_CHARGING) >> DS_STATUS_CHARGING_SHIFT;
        let battery_status = _get_battery_status(charging_status, battery_data);

        let json = match serde_json::to_string(&battery_status) {
          Ok(x) => x,
          Err(err) => {
              error!(
                  "get_running_game failed due an error serializing game json: {}",
                  err
              );
              return Vec::new();
          }
      };
      return vec![Primitive::Json(json)]
    }
  }
  return Vec::new();
}

fn _get_battery_status(charging_status: u8, battery_data: u8) -> BatteryInfo {
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
          capacity: 100,
          status: "full".to_string(),
      },
      0x2 => BatteryInfo {
          capacity: min(battery_data * 10 + 5, 100),
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