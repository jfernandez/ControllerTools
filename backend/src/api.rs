mod gulikit;
mod nintendo;
mod playstation;
mod xbox;
use anyhow::Result;
use hidapi::HidApi;
use log::{debug, error};
use serde::{Deserialize, Serialize};
use udev::Enumerator;

use std::fs;
use std::path::Path;
use std::slice::IterMut;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Controller {
    pub name: String,
    pub product_id: u16,
    pub vendor_id: u16,
    pub capacity: u8,
    pub status: String,
    pub bluetooth: bool,
    pub hid_device_path: String,
}

fn get_battery_capacity(device_path: &Path) -> Result<u8> {
    let s = fs::read_dir([device_path.to_str().unwrap(), "/device/power_supply"].join("/"))?
        .next()
        .unwrap()?;
    debug!("Power supply path: {:?}", s);
    let capacity_string = fs::read_to_string([s.path().to_str().unwrap(), "capacity"].join("/"));
    let capacity: u8 = capacity_string?.trim().parse()?;
    debug!("Found battery capacity: {}", capacity);
    return Ok(capacity);
}

fn get_missing_battery_capacity(controllers: IterMut<Controller>) -> Result<()> {
    let mut bt_enum = Enumerator::new()?;
    // bt_enum.match_subsystem("hid")?;
    bt_enum.match_subsystem("hidraw")?;
    for controller in controllers {
        if controller.capacity == 0 && controller.hid_device_path != "unknown".to_string() {
            for device in bt_enum.scan_devices()? {
                if device.devnode().unwrap().to_str().unwrap() == controller.hid_device_path {
                    debug!(
                        "Trying to get battery capacity for controller: {}",
                        controller.name
                    );
                    let device_path = device.syspath();
                    match get_battery_capacity(device_path) {
                        Ok(capacity) => {
                            controller.capacity = capacity;
                            if controller.status == "unknown".to_string() {
                                controller.status = "discharging".to_string();
                            }
                        }
                        Err(err) => error!("Error getting capacity: {:?}", err),
                    }
                    // let capacity = get_battery_capacity(device_path)?;
                    // controller.capacity = capacity;
                    // if controller.status == "unknown".to_string() {
                    //     controller.status = "discharging".to_string();
                    // }
                }
            }
        } else {
            debug!(
                "Capacity known for controller {}: {} ({})",
                controller.name, controller.capacity, controller.status
            );
        }
    }
    return Ok(());
}

pub fn get_controllers() -> Result<Vec<Controller>> {
    let hidapi = HidApi::new()?;
    let mut controllers: Vec<Controller> = Vec::new();

    // HidApi will return 2 copies of the device when the Nintendo Pro Controller is connected via USB.
    // It will additionally return a 3rd device when the controller is connected via Bluetooth + USB.
    let nintendo_pro_controllers: Vec<_> = hidapi
        .device_list()
        .filter(|device_info| {
            device_info.vendor_id() == nintendo::VENDOR_ID_NINTENDO
                && device_info.product_id() == nintendo::PRODUCT_ID_NINTENDO_PROCON
        })
        .collect();

    if nintendo_pro_controllers.len() == 1 {
        // When we only get one device, we know it's connected via Bluetooth.
        let controller =
            nintendo::parse_pro_controller_data(&nintendo_pro_controllers[0], &hidapi)?;
        controllers.push(controller);
    } else if nintendo_pro_controllers.len() == 2 {
        // When we get two devices, we know it's connected only via USB. Both will report the same data, so we'll just return the first one.
        let controller =
            nintendo::parse_pro_controller_data(&nintendo_pro_controllers[0], &hidapi)?;
        controllers.push(controller);
    } else if nintendo_pro_controllers.len() == 3 {
        // When we get three devices, we know it's connected via USB + Bluetooth.
        // We'll only return the Bluetooth device because the USB devices will not report any data.
        let bt_controller = nintendo_pro_controllers
            .iter()
            .find(|device_info| device_info.interface_number() == -1);

        if let Some(bt_controller) = bt_controller {
            let controller = nintendo::parse_pro_controller_data(&bt_controller, &hidapi)?;
            controllers.push(controller);
        }
    }

    // for some reason HidApi's list_devices() is returning multiple instances of the same controller
    // so dedupe by serial number
    let mut xbox_controllers: Vec<_> = hidapi
        .device_list()
        .filter(|device_info| {
            device_info.vendor_id() == xbox::MS_VENDOR_ID
                && (device_info.product_id() == xbox::XBOX_CONTROLLER_PRODUCT_ID
                    || device_info.product_id() == xbox::XBOX_WIRELESS_CONTROLLER_USB_PRODUCT_ID
                    || device_info.product_id() == xbox::XBOX_WIRELESS_CONTROLLER_BT_PRODUCT_ID)
        })
        .collect();
    xbox_controllers.dedup_by(|a, b| a.serial_number() == b.serial_number());
    for device_info in xbox_controllers {
        match (device_info.vendor_id(), device_info.product_id()) {
            (xbox::MS_VENDOR_ID, xbox::XBOX_CONTROLLER_PRODUCT_ID) => {
                debug!("!Found Xbox One S controller: {:?}", device_info);
                let controller = xbox::parse_xbox_controller_data(&device_info, &hidapi)?;
                controllers.push(controller);
            }
            // (xbox::MS_VENDOR_ID, xbox::XBOX_WIRELESS_CONTROLLER_USB_PRODUCT_ID) => {
            //     debug!("Found Xbox Series X/S controller: {:?}", device_info);
            //     let controller = xbox::parse_xbox_controller_data(&device_info, &hidapi)?;

            //     controllers.push(controller);
            // }
            (xbox::MS_VENDOR_ID, xbox::XBOX_WIRELESS_CONTROLLER_BT_PRODUCT_ID) => {
                debug!("Found Xbox Series X/S controller: {:?}", device_info);
                let controller = xbox::parse_xbox_controller_data(&device_info, &hidapi)?;

                controllers.push(controller);
            }
            _ => {}
        }
    }

    let mut gulikit_controllers: Vec<_> = hidapi
        .device_list()
        .filter(|device_info| {
            device_info.vendor_id() == gulikit::VENDOR_ID
                && device_info.product_id() == gulikit::PRODUCT_ID
        })
        .collect();
    gulikit_controllers.dedup_by(|a, b| a.serial_number() == b.serial_number());

    for device_info in gulikit_controllers {
        match (device_info.vendor_id(), device_info.product_id()) {
            (gulikit::VENDOR_ID, gulikit::PRODUCT_ID) => {
                if device_info.interface_number() == -1 {
                    debug!("Found GuliKit controller: {:?}", device_info);
                    let controller = gulikit::parse_controller_data(&device_info, &hidapi)?;
                    controllers.push(controller);
                }
            }
            _ => {}
        }
    }

    for device_info in hidapi.device_list() {
        match (device_info.vendor_id(), device_info.product_id()) {
            (playstation::DS_VENDOR_ID, playstation::DS_PRODUCT_ID) => {
                debug!("Found DualSense controller: {:?}", device_info);
                let controller =
                    playstation::parse_dualsense_controller_data(&device_info, &hidapi)?;

                controllers.push(controller);
            }
            (playstation::DS_VENDOR_ID, playstation::DS4_NEW_PRODUCT_ID) => {
                debug!("Found new DualShock 4 controller: {:?}", device_info);
                let controller =
                    playstation::parse_dualshock_controller_data(&device_info, &hidapi)?;

                controllers.push(controller);
            }
            (playstation::DS_VENDOR_ID, playstation::DS4_OLD_PRODUCT_ID) => {
                debug!("Found old DualShock 4 controller: {:?}", device_info);
                let controller =
                    playstation::parse_dualshock_controller_data(&device_info, &hidapi)?;

                controllers.push(controller);
            }
            _ => {}
        }
    }

    // for Xbox over USB, hidapi-rs is not finding controllers so fall back to using udev
    let mut enumerator = Enumerator::new()?;
    enumerator.match_subsystem("usb")?;
    for device in enumerator.scan_devices()? {
        let device_vendor_id = match device.property_value("ID_VENDOR_ID") {
            Some(val) => val.to_str().unwrap(),
            None => "0",
        };
        let device_product_id = match device.property_value("ID_MODEL_ID") {
            Some(val) => val.to_str().unwrap(),
            None => "0",
        };
        if device_vendor_id == xbox::MS_VENDOR_ID_STR {
            if device_product_id == xbox::XBOX_CONTROLLER_USB_PRODUCT_ID_STR {
                debug!("Found Xbox One S controller over USB");
                controllers.push(xbox::get_xbox_controller(
                    xbox::XBOX_CONTROLLER_USB_PRODUCT_ID,
                    false,
                )?)
            } else if device_product_id == xbox::XBOX_WIRELESS_CONTROLLER_USB_PRODUCT_ID_STR {
                debug!("Found Xbox Series X/S controller over USB");
                controllers.push(xbox::get_xbox_controller(
                    xbox::XBOX_WIRELESS_CONTROLLER_USB_PRODUCT_ID,
                    false,
                )?)
            }
        }
    }

    // let mut bt_enum = Enumerator::new()?;
    // // bt_enum.match_subsystem("hid")?;
    // bt_enum.match_subsystem("hidraw")?;
    // bt_enum.match_subsystem("bluetooth")?;
    // let udev_devices = ;
    // for device in bt_enum.scan_devices()? {
    //     if device.devnode().unwrap().to_str().unwrap() == test_devpath {
    //         let capacity = get_battery_capacity(device.syspath())?;
    //         // let p = Path::new(test_devpath);
    //         // println!("Found {} ==> {:?}", test_devpath, device.syspath());
    //         // let s = fs::read_dir(
    //         //     [device.syspath().to_str().unwrap(), "/device/power_supply"].join("/"),
    //         // )?
    //         // .next()
    //         // .unwrap()?;
    //         // println!("Got filename {:?}", s.path().to_str());
    //         // let capacity_string =
    //         //     fs::read_to_string([s.path().to_str().unwrap(), "capacity"].join("/"));
    //         // let capacity: u8 = capacity_string?.trim().parse()?;
    //         // println!("!!battery capacity: {:?}", capacity);
    //     }
    // }
    // controllers = controllers.to_vec();
    // unsafe {
    //     for controller in controllers.iter_mut() {
    //         if controller.capacity == 0 && controller.hid_device_path != "unknown".to_string() {
    //             for device in bt_enum.scan_devices()? {
    //                 if device.devnode().unwrap().to_str().unwrap() == controller.hid_device_path {
    //                     let device_path = device.syspath();
    //                     let capacity = get_battery_capacity(device_path)?;
    //                     controller.capacity = capacity;
    //                 }
    //             }
    //         }
    //     }
    // }

    match get_missing_battery_capacity(controllers.iter_mut()) {
        Ok(_) => {}
        Err(_) => error!("Failed to get missing battery status"),
    }

    return Ok(controllers);
}
