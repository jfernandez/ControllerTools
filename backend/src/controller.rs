use std::ffi::OsStr;

use hidapi::DeviceInfo;
use log::error;
use serde::{Deserialize, Serialize};
use udev::Device;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Charging,
    Discharging,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Controller {
    pub name: String,
    pub product_id: u16,
    pub vendor_id: u16,
    pub capacity: u8,
    pub status: Status,
    pub bluetooth: bool,
    #[serde(skip_serializing)]
    pub serial_number: Option<String>,
    #[serde(skip_serializing)]
    pub device_path: Option<String>,
}

impl Controller {
    pub fn from_udev(
        device: &Device,
        name: &str,
        capacity: u8,
        status: Status,
        bluetooth: bool,
    ) -> Self {
        let serial_number = device
            .property_value("ID_SERIAL_SHORT")
            .map(|serial_number| serial_number.to_string_lossy().to_string());
        let device_path = if device.devpath().is_empty() {
            None
        } else {
            Some(device.devpath().to_string_lossy().to_string())
        };

        let vendor_id: u16 = device
            .property_value("ID_VENDOR_ID")
            .map(hex_os_str_to_u16)
            .unwrap_or(0);
        let product_id: u16 = device
            .property_value("ID_MODEL_ID")
            .map(hex_os_str_to_u16)
            .unwrap_or(0);

        Self {
            name: name.to_string(),
            vendor_id,
            product_id,
            capacity,
            status,
            bluetooth,
            serial_number,
            device_path,
        }
    }

    pub fn from_hidapi(device_info: &DeviceInfo, name: &str, capacity: u8, status: Status) -> Self {
        let serial_number = device_info
            .serial_number()
            .filter(|serial_number| !serial_number.is_empty())
            .map(|serial_number| serial_number.to_string());
        let bluetooth = device_info.interface_number() == -1;
        let device_path_bytes = device_info.path().to_bytes();
        let device_path = if device_path_bytes.is_empty() {
            None
        } else {
            Some(String::from_utf8_lossy(device_path_bytes).to_string())
        };

        Self {
            name: name.to_string(),
            product_id: device_info.product_id(),
            vendor_id: device_info.vendor_id(),
            capacity,
            status,
            bluetooth,
            serial_number,
            device_path,
        }
    }

    pub fn id(&self) -> String {
        // Use the device path if it's available, otherwise use the serial number.
        // If neither are available, use a combination of the vendor and product IDs.
        match &self.device_path {
            Some(device_path) => device_path.to_string(),
            None => match &self.serial_number {
                Some(serial_number) => serial_number.to_string(),
                None => format!("{}:{}", self.vendor_id, self.product_id),
            },
        }
    }

    pub fn is_discharging(&self) -> bool {
        self.status == Status::Discharging
    }
}

fn hex_os_str_to_u16(hex_os_str: &OsStr) -> u16 {
    let hex_str = hex_os_str.to_string_lossy();

    match u16::from_str_radix(&hex_str, 16) {
        Ok(num) => num,
        Err(err) => {
            error!("Failed to parse hex string: {}", err);
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{hex_os_str_to_u16, Controller, Status};
    use std::ffi::OsStr;

    #[test]
    fn test_is_discharging() {
        let mut controller = Controller {
            name: "Test Controller".to_string(),
            product_id: 0x045e,
            vendor_id: 0x02ea,
            capacity: 0,
            status: Status::Discharging,
            bluetooth: false,
            serial_number: None,
            device_path: None,
        };
        assert!(controller.is_discharging());

        controller.status = Status::Charging;
        assert!(!controller.is_discharging());

        controller.status = Status::Unknown;
        assert!(!controller.is_discharging());
    }

    #[test]
    fn test_json_serialization() {
        // Verify that serde doesn't serialize the serial_number and device_path fields
        let controller = Controller {
            name: "Test Controller".to_string(),
            product_id: 0x045e,
            vendor_id: 0x02ea,
            capacity: 0,
            status: Status::Discharging,
            bluetooth: false,
            serial_number: Some("1234567890".to_string()),
            device_path: Some("/dev/input/js0".to_string()),
        };
        let serialized = serde_json::to_string(&controller).unwrap();
        assert_eq!(
            serialized,
            r#"{"name":"Test Controller","productId":1118,"vendorId":746,"capacity":0,"status":"discharging","bluetooth":false}"#
        );
    }

    #[test]
    fn test_id() {
        let mut controller = Controller {
            name: "Test Controller".to_string(),
            product_id: 0x045e,
            vendor_id: 0x02ea,
            capacity: 0,
            status: Status::Discharging,
            bluetooth: false,
            device_path: Some("/dev/input/js0".to_string()),
            serial_number: Some("1234567890".to_string()),
        };

        assert_eq!(controller.id(), "/dev/input/js0");
        controller.device_path = None;
        assert_eq!(controller.id(), "1234567890");
        controller.serial_number = None;
        assert_eq!(controller.id(), "746:1118");
    }

    #[test]
    fn test_hex_os_str_to_u16() {
        let os_str = OsStr::new("045e");
        let parsed_num = hex_os_str_to_u16(os_str);
        assert_eq!(0x045e, parsed_num);
        assert_eq!(1118, parsed_num);

        let os_str = OsStr::new("02ea");
        let parsed_num = hex_os_str_to_u16(os_str);
        assert_eq!(0x02ea, parsed_num);
        assert_eq!(746, parsed_num);

        let os_str = OsStr::new("foobar");
        let parsed_num = hex_os_str_to_u16(os_str);
        assert_eq!(0, parsed_num);
    }
}
