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

        for device_info in hidapi.device_list() {
            match (device_info.vendor_id(), device_info.product_id()) {
                (playstation::DS_VENDOR_ID, playstation::DS_PRODUCT_ID) => {
                    debug!("Found DualSense controller: {:?}", device_info);
                    let controller =
                        playstation::parse_dualsense_controller_data(&device_info, &hidapi)?;

                    controllers.push(controller);
                }
                _ => {}
            }
        }
        return Ok(controllers);
    }
}
