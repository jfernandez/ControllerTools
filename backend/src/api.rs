mod nintendo;
mod playstation;
use anyhow::Result;
use hidapi::HidApi;
use log::debug;
use serde::{Deserialize, Serialize};

pub struct API;
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Controller {
    pub name: String,
    pub product_id: u16,
    pub vendor_id: u16,
    pub capacity: u8,
    pub status: String,
    pub bluetooth: bool,
}

impl API {
    pub fn get_controllers(&self) -> Result<Vec<Controller>> {
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
        return Ok(controllers);
    }
}
