use evdev::{Device, InputEventKind};
use std::path::Path;


fn main() {


    let device_path = Path::new("/dev/input/event3");


  // Open the input device
    let mut device = match Device::open(device_path) {
        Ok(dev) => dev,
        Err(err) => {
            eprintln!("Failed to open device: {}", err);
            return;
        },
    };

        println!("Listening for events on: {}", device.name().unwrap_or("Unknown device"));


}
