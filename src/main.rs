use evdev::{Device, InputEventKind};
use std::path::Path;

fn log_keystroke<const N: usize>(
    c: char,
    buffer: &mut [char; N],
    start: &mut usize,
    end: &mut usize,
) {
}

fn main() {
    // this should eventually be managed by an external file under user control
    let shortcuts = [(
        "palt",
        "Please generate an alt-text description for this image.",
    )];

    // a circular buffer to store keystrokes...it's length should be equal to the length of the longest shortcut but elt's hardcode for now
    let mut buffer: [char; 4];
    let mut buffer_start: usize = 0;
    let mut buffer_end: usize = 0;

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
                    println!(
                        "Key event: {:?} - State: {} - Code: {}",
                        key,
                        ev.value(),
                        ev.code()
                    );
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

fn evdev_key_to_char(key: evdev::Key) -> Option<char> {
    match key {
        evdev::Key::KEY_A => Some('a'),
        evdev::Key::KEY_B => Some('b'),
        evdev::Key::KEY_C => Some('c'),
        evdev::Key::KEY_D => Some('d'),
        evdev::Key::KEY_E => Some('e'),
        evdev::Key::KEY_F => Some('f'),
        evdev::Key::KEY_G => Some('g'),
        evdev::Key::KEY_H => Some('h'),
        evdev::Key::KEY_I => Some('i'),
        evdev::Key::KEY_J => Some('j'),
        evdev::Key::KEY_K => Some('k'),
        evdev::Key::KEY_L => Some('l'),
        evdev::Key::KEY_M => Some('m'),
        evdev::Key::KEY_N => Some('n'),
        evdev::Key::KEY_O => Some('o'),
        evdev::Key::KEY_P => Some('p'),
        evdev::Key::KEY_Q => Some('q'),
        evdev::Key::KEY_R => Some('r'),
        evdev::Key::KEY_S => Some('s'),
        evdev::Key::KEY_T => Some('t'),
        evdev::Key::KEY_U => Some('u'),
        evdev::Key::KEY_V => Some('v'),
        evdev::Key::KEY_W => Some('w'),
        evdev::Key::KEY_X => Some('x'),
        evdev::Key::KEY_Y => Some('y'),
        evdev::Key::KEY_Z => Some('z'),
        evdev::Key::KEY_1 => Some('1'),
        evdev::Key::KEY_2 => Some('2'),
        evdev::Key::KEY_3 => Some('3'),
        evdev::Key::KEY_4 => Some('4'),
        evdev::Key::KEY_5 => Some('5'),
        evdev::Key::KEY_6 => Some('6'),
        evdev::Key::KEY_7 => Some('7'),
        evdev::Key::KEY_8 => Some('8'),
        evdev::Key::KEY_9 => Some('9'),
        evdev::Key::KEY_0 => Some('0'),
        evdev::Key::KEY_SPACE => Some(' '),
        evdev::Key::KEY_MINUS => Some('-'),
        evdev::Key::KEY_EQUAL => Some('='),
        evdev::Key::KEY_LEFTBRACE => Some('['),
        evdev::Key::KEY_RIGHTBRACE => Some(']'),
        evdev::Key::KEY_BACKSLASH => Some('\\'),
        evdev::Key::KEY_SEMICOLON => Some(';'),
        evdev::Key::KEY_APOSTROPHE => Some('\''),
        evdev::Key::KEY_COMMA => Some(','),
        evdev::Key::KEY_DOT => Some('.'),
        evdev::Key::KEY_SLASH => Some('/'),
        _ => None, // Return None for unmapped keys
    }
}

#[cfg(test)]
#[test]
fn test_buffer_impl() {
    let mut buffer: [char; 4] = ['_', '_', '_', '_'];
    let mut buffer_start: usize = 0;
    let mut buffer_end: usize = 0;

    log_keystroke('p', &mut buffer, &mut buffer_start, &mut buffer_end);
    assert_eq!('p', buffer[0]);
}
