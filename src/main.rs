use evdev::{Device, InputEventKind};
use std::fs::OpenOptions;
use std::os::fd::AsFd;
use std::os::fd::AsRawFd;
use std::path::Path;
use std::{thread, time::Duration};
use uinput::event::keyboard::Key;
use uinput::event::Keyboard::All;
use uinput::Device as UInputDevice;

fn log_keystroke<const N: usize>(c: char, buffer: &mut [char; N], head: &mut usize) {
    buffer[*head] = c;
    *head = (*head + 1) % N;
}

fn buffer_to_string<const N: usize>(buffer: [char; N], head: usize) -> String {
    let mut result = String::new();

    let mut index = head % N;

    loop {
        result.push(buffer[index]);
        if index == (head + (N - 1)) % N {
            break;
        }
        index = (index + 1) % N;
    }
    return result;
}

fn buffer_matches<const N: usize>(buffer: [char; N], head: usize, alias: String) -> bool {
    let buffer_str = buffer_to_string(buffer, head); //TODO: can replace string creation here by directly iterating on buffer
    return buffer_str.ends_with(&alias);
}

fn main() {
    // this should eventually be managed by an external file under user control
    let shortcuts = [
        (
            "palt",
            "Please generate an alt-text description for this image.",
        ),
        ("tm ", "™"),
        ("foo", "bar"),
        ("omw", "On my way!"),
    ];

    // a circular buffer to store keystrokes...it's length should be equal to the length of the longest shortcut but elt's hardcode for now
    let mut buffer: [char; 4] = ['\0', '\0', '\0', '\0'];
    let mut buffer_head: usize = 0;

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

    //open the vurtual input device for writing replacements
    let uinput_file = match OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/uinput")
    {
        Ok(dev) => dev,
        Err(err) => {
            eprintln!("Failed to open device: {}", err);
            return;
        }
    }; // Opens the uinput device file
    let uinput_fd = uinput_file.as_raw_fd(); // Gets the raw file descriptor (i32)

    println!("uinput file descriptor: {:?}", uinput_fd);

    println!(
        "Listening for events on: {}",
        device.name().unwrap_or("Unknown device")
    );

    let mut uinput_device = uinput::open("/dev/uinput")
        .unwrap()
        .name("Virtual Keyboard")
        .unwrap()
        .event(All)
        .unwrap()
        .create()
        .unwrap(); // Add events.

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
                    if (ev.value() == 0) {
                        log_keystroke(
                            key_to_char(key).expect("invalid keystroke...could not map char"),
                            &mut buffer,
                            &mut buffer_head,
                        );

                        for alias in shortcuts {
                            if (buffer_matches(buffer, buffer_head, alias.0.to_string())) {
                                println!("Found match! Replacing {} with {}", alias.0, alias.1);
                                for alias_char in alias.0.chars() {
                                    uinput_device.press(&Key::BackSpace);
                                    uinput_device.release(&Key::BackSpace);
                                }
                                uinput_device.synchronize();
                                for substitution_char in alias.1.chars() {
                                    let subchar = char_to_key(substitution_char);
                                    println!("Printing  {}", substitution_char);
                                    if subchar != None {
                                        uinput_device.press(&subchar.unwrap());
                                        uinput_device.release(&subchar.unwrap());
                                    }
                                }
                                uinput_device.synchronize();
                                break;
                            }
                            println!(
                                "Did not match  {} against {}",
                                alias.0,
                                buffer_to_string(buffer, buffer_head)
                            );
                        }
                    }
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

fn key_to_char(key: evdev::Key) -> Option<char> {
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
        _ => Some('\0'), // Return nul character for unmapped keys
    }
}

fn char_to_key(ch: char) -> Option<Key> {
    match ch.to_lowercase().next().unwrap() {
        'a' => Some(Key::A),
        'b' => Some(Key::B),
        'c' => Some(Key::C),
        'd' => Some(Key::D),
        'e' => Some(Key::E),
        'f' => Some(Key::F),
        'g' => Some(Key::G),
        'h' => Some(Key::H),
        'i' => Some(Key::I),
        'j' => Some(Key::J),
        'k' => Some(Key::K),
        'l' => Some(Key::L),
        'm' => Some(Key::M),
        'n' => Some(Key::N),
        'o' => Some(Key::O),
        'p' => Some(Key::P),
        'q' => Some(Key::Q),
        'r' => Some(Key::R),
        's' => Some(Key::S),
        't' => Some(Key::T),
        'u' => Some(Key::U),
        'v' => Some(Key::V),
        'w' => Some(Key::W),
        'x' => Some(Key::X),
        'y' => Some(Key::Y),
        'z' => Some(Key::Z),
        '1' => Some(Key::_1),
        '2' => Some(Key::_2),
        '3' => Some(Key::_3),
        '4' => Some(Key::_4),
        '5' => Some(Key::_5),
        '6' => Some(Key::_6),
        '7' => Some(Key::_7),
        '8' => Some(Key::_8),
        '9' => Some(Key::_9),
        '0' => Some(Key::_0),
        ' ' => Some(Key::Space),
        '-' => Some(Key::Minus),
        '=' => Some(Key::Equal),
        '[' => Some(Key::LeftBrace),
        ']' => Some(Key::RightBrace),
        '\\' => Some(Key::BackSlash),
        ';' => Some(Key::SemiColon),
        '\'' => Some(Key::Apostrophe),
        ',' => Some(Key::Comma),
        '.' => Some(Key::Dot),
        '/' => Some(Key::Slash),
        _ => None, // Return None for unmapped characters
    }
}

#[cfg(test)]
#[test]
fn test_buffer_matches() {
    let mut buffer: [char; 4] = ['\0', '\0', '\0', '\0'];
    let mut buffer_head: usize = 0;

    log_keystroke('p', &mut buffer, &mut buffer_head);
    log_keystroke('a', &mut buffer, &mut buffer_head);
    log_keystroke('l', &mut buffer, &mut buffer_head);
    log_keystroke('t', &mut buffer, &mut buffer_head);

    assert!(buffer_matches(buffer, buffer_head, "palt".to_string()));

    log_keystroke('o', &mut buffer, &mut buffer_head);
    log_keystroke('m', &mut buffer, &mut buffer_head);
    log_keystroke('w', &mut buffer, &mut buffer_head);

    assert!(buffer_matches(buffer, buffer_head, "omw".to_string()));

    log_keystroke('t', &mut buffer, &mut buffer_head);
    log_keystroke('m', &mut buffer, &mut buffer_head);
    log_keystroke(' ', &mut buffer, &mut buffer_head);

    assert!(buffer_matches(buffer, buffer_head, "tm ".to_string()));

    assert!(!buffer_matches(buffer, buffer_head, "wtm".to_string()));
}

#[test]
fn test_buffer_impl() {
    let mut buffer: [char; 4] = ['\0', '\0', '\0', '\0'];
    let mut buffer_head: usize = 0;

    log_keystroke('p', &mut buffer, &mut buffer_head);
    assert_eq!('p', buffer[0]);
    assert_eq!(1, buffer_head);

    log_keystroke('a', &mut buffer, &mut buffer_head);
    assert_eq!(2, buffer_head);

    log_keystroke('l', &mut buffer, &mut buffer_head);
    assert_eq!(3, buffer_head);

    log_keystroke('t', &mut buffer, &mut buffer_head);
    assert_eq!(0, buffer_head);

    log_keystroke('x', &mut buffer, &mut buffer_head);
    assert_eq!('x', buffer[0]);
    assert_eq!(1, buffer_head);
}

#[test]
fn test_buffer_to_string() {
    let mut buffer: [char; 4] = ['\0', '\0', '\0', '\0'];
    let mut buffer_head: usize = 0;

    assert_eq!("\0\0\0\0", buffer_to_string(buffer, buffer_head));

    log_keystroke('p', &mut buffer, &mut buffer_head);
    log_keystroke('a', &mut buffer, &mut buffer_head);
    log_keystroke('l', &mut buffer, &mut buffer_head);
    log_keystroke('t', &mut buffer, &mut buffer_head);

    assert_eq!("palt", buffer_to_string(buffer, buffer_head));

    log_keystroke('o', &mut buffer, &mut buffer_head);
    log_keystroke('m', &mut buffer, &mut buffer_head);
    log_keystroke('w', &mut buffer, &mut buffer_head);

    assert_eq!("tomw", buffer_to_string(buffer, buffer_head));

    log_keystroke(' ', &mut buffer, &mut buffer_head);
    log_keystroke('t', &mut buffer, &mut buffer_head);
    log_keystroke('m', &mut buffer, &mut buffer_head);

    assert_eq!("w tm", buffer_to_string(buffer, buffer_head));
}
