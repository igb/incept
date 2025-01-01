use evdev::{Device, InputEventKind};
use std::path::Path;

fn main() {
    // this should eventually be managed by an external file under user control
    let shortcuts = [(
        "palt",
        "Please generate an alt-text description for this image.",
    )];

    // a circular buffer to store keystrokes...it's length should be equal to the length of the longest shortcut but elt's hardcode for now
    let mut buffer: [char; 4];

    // need to make this dynamic
    let device_path = Path::new("/dev/input/event3");

    // Open the input device
    let mut device = match Device::open(device_path) {
        Ok(dev) => dev,
        Err(err) => {
            eprintln!("Failed to open device: {}", err);
            return;
        }
    };

    println!(
        "Listening for events on: {}",
        device.name().unwrap_or("Unknown device")
    );

    // Event loop: read and process events
    loop {
        for ev in device.fetch_events().unwrap() {
            match ev.kind() {
                InputEventKind::Key(key) => {
                    println!("Key event: {:?} - State: {}", key, ev.value());
                }
                InputEventKind::RelAxis(axis) => {
                    println!("Relative axis event: {:?} - Value: {}", axis, ev.value());
                }
                InputEventKind::AbsAxis(axis) => {
                    println!("Absolute axis event: {:?} - Value: {}", axis, ev.value());
                }
                _ => {
                    println!("Other event: {:?}", ev);
                }
            }
        }
    }
}
