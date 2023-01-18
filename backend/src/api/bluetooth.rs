use anyhow::Result;
use hidapi::DeviceInfo;
use std::io::BufRead;
use std::{fs::File, io, path::Path, process::Command};

/// Get the bluetooth address from the DeviceInfo's hidraw,
/// e.g. "/sys/class/hidraw/hidraw5/device/uevent".
/// This file contains the BT address as value of HID_UNIQ
pub fn get_bluetooth_address(device_info: &DeviceInfo) -> Result<String> {
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
pub fn get_battery_percentage(address: String) -> Result<u8> {
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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
