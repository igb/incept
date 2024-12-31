use evdev::{Device, InputEventKind};
use std::path::Path;
use uinput::Device as UInputDevice;

fn main() {
   // Open the input device
    let input_device = Device::open(Path::new("/dev/input/event0")).expect("Failed to open device");


}
