use std::os::raw::c_ushort;

use cocoa::{
    appkit::{NSEvent, NSEventModifierFlags},
    base::id,
};

use crate::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, ModifiersState, VirtualKeyCode, WindowEvent},
    platform_impl::platform::{
        util::{IdRef, Never},
        DEVICE_ID,
    },
};

#[derive(Debug)]
pub enum EventWrapper {
    StaticEvent(Event<'static, Never>),
    EventProxy(EventProxy),
}

#[derive(Debug, PartialEq)]
pub enum EventProxy {
    DpiChangedProxy {
        ns_window: IdRef,
        suggested_size: LogicalSize<f64>,
        scale_factor: f64,
    },
}

pub fn char_to_keycode(c: char) -> Option<VirtualKeyCode> {
    // We only translate keys that are affected by keyboard layout.
    //
    // Note that since keys are translated in a somewhat "dumb" way (reading character)
    // there is a concern that some combination, i.e. Cmd+char, causes the wrong
    // letter to be received, and so we receive the wrong key.
    //
    // Implementation reference: https://github.com/WebKit/webkit/blob/82bae82cf0f329dbe21059ef0986c4e92fea4ba6/Source/WebCore/platform/cocoa/KeyEventCocoa.mm#L626
    Some(match c {
        'a' | 'A' => VirtualKeyCode::A,
        'b' | 'B' => VirtualKeyCode::B,
        'c' | 'C' => VirtualKeyCode::C,
        'd' | 'D' => VirtualKeyCode::D,
        'e' | 'E' => VirtualKeyCode::E,
        'f' | 'F' => VirtualKeyCode::F,
        'g' | 'G' => VirtualKeyCode::G,
        'h' | 'H' => VirtualKeyCode::H,
        'i' | 'I' => VirtualKeyCode::I,
        'j' | 'J' => VirtualKeyCode::J,
        'k' | 'K' => VirtualKeyCode::K,
        'l' | 'L' => VirtualKeyCode::L,
        'm' | 'M' => VirtualKeyCode::M,
        'n' | 'N' => VirtualKeyCode::N,
        'o' | 'O' => VirtualKeyCode::O,
        'p' | 'P' => VirtualKeyCode::P,
        'q' | 'Q' => VirtualKeyCode::Q,
        'r' | 'R' => VirtualKeyCode::R,
        's' | 'S' => VirtualKeyCode::S,
        't' | 'T' => VirtualKeyCode::T,
        'u' | 'U' => VirtualKeyCode::U,
        'v' | 'V' => VirtualKeyCode::V,
        'w' | 'W' => VirtualKeyCode::W,
        'x' | 'X' => VirtualKeyCode::X,
        'y' | 'Y' => VirtualKeyCode::Y,
        'z' | 'Z' => VirtualKeyCode::Z,
        '1' | '!' => VirtualKeyCode::Key1,
        '2' | '@' => VirtualKeyCode::Key2,
        '3' | '#' => VirtualKeyCode::Key3,
        '4' | '$' => VirtualKeyCode::Key4,
        '5' | '%' => VirtualKeyCode::Key5,
        '6' | '^' => VirtualKeyCode::Key6,
        '7' | '&' => VirtualKeyCode::Key7,
        '8' | '*' => VirtualKeyCode::Key8,
        '9' | '(' => VirtualKeyCode::Key9,
        '0' | ')' => VirtualKeyCode::Key0,
        '=' | '+' => VirtualKeyCode::Equals,
        '-' | '_' => VirtualKeyCode::Minus,
        ']' | '}' => VirtualKeyCode::RBracket,
        '[' | '{' => VirtualKeyCode::LBracket,
        '\'' | '"' => VirtualKeyCode::Apostrophe,
        ';' | ':' => VirtualKeyCode::Semicolon,
        '\\' | '|' => VirtualKeyCode::Backslash,
        ',' | '<' => VirtualKeyCode::Comma,
        '/' | '?' => VirtualKeyCode::Slash,
        '.' | '>' => VirtualKeyCode::Period,
        '`' | '~' => VirtualKeyCode::Grave,
        _ => return None,
    })
}

pub fn keycode_to_char(keycode: VirtualKeyCode, modifiers_state: ModifiersState) -> Option<char> {
    // Reverse translation of keycode to char, based on char_to_keycode
    Some(match (keycode, modifiers_state.contains(ModifiersState::SHIFT)) {
        (VirtualKeyCode::A, false) => 'a',
        (VirtualKeyCode::A, true) => 'A',
        (VirtualKeyCode::B, false) => 'b',
        (VirtualKeyCode::B, true) => 'B',
        (VirtualKeyCode::C, false) => 'c',
        (VirtualKeyCode::C, true) => 'C',
        (VirtualKeyCode::D, false) => 'd',
        (VirtualKeyCode::D, true) => 'D',
        (VirtualKeyCode::E, false) => 'e',
        (VirtualKeyCode::E, true) => 'E',
        (VirtualKeyCode::F, false) => 'f',
        (VirtualKeyCode::F, true) => 'F',
        (VirtualKeyCode::G, false) => 'g',
        (VirtualKeyCode::G, true) => 'G',
        (VirtualKeyCode::H, false) => 'h',
        (VirtualKeyCode::H, true) => 'H',
        (VirtualKeyCode::I, false) => 'i',
        (VirtualKeyCode::I, true) => 'I',
        (VirtualKeyCode::J, false) => 'j',
        (VirtualKeyCode::J, true) => 'J',
        (VirtualKeyCode::K, false) => 'k',
        (VirtualKeyCode::K, true) => 'K',
        (VirtualKeyCode::L, false) => 'l',
        (VirtualKeyCode::L, true) => 'L',
        (VirtualKeyCode::M, false) => 'm',
        (VirtualKeyCode::M, true) => 'M',
        (VirtualKeyCode::N, false) => 'n',
        (VirtualKeyCode::N, true) => 'N',
        (VirtualKeyCode::O, false) => 'o',
        (VirtualKeyCode::O, true) => 'O',
        (VirtualKeyCode::P, false) => 'p',
        (VirtualKeyCode::P, true) => 'P',
        (VirtualKeyCode::Q, false) => 'q',
        (VirtualKeyCode::Q, true) => 'Q',
        (VirtualKeyCode::R, false) => 'r',
        (VirtualKeyCode::R, true) => 'R',
        (VirtualKeyCode::S, false) => 's',
        (VirtualKeyCode::S, true) => 'S',
        (VirtualKeyCode::T, false) => 't',
        (VirtualKeyCode::T, true) => 'T',
        (VirtualKeyCode::U, false) => 'u',
        (VirtualKeyCode::U, true) => 'U',
        (VirtualKeyCode::V, false) => 'v',
        (VirtualKeyCode::V, true) => 'V',
        (VirtualKeyCode::W, false) => 'w',
        (VirtualKeyCode::W, true) => 'W',
        (VirtualKeyCode::X, false) => 'x',
        (VirtualKeyCode::X, true) => 'X',
        (VirtualKeyCode::Y, false) => 'y',
        (VirtualKeyCode::Y, true) => 'Y',
        (VirtualKeyCode::Z, false) => 'z',
        (VirtualKeyCode::Z, true) => 'Z',
        (VirtualKeyCode::Key1, false) => '1',
        (VirtualKeyCode::Key1, true) => '!',
        (VirtualKeyCode::Key2, false) => '2',
        (VirtualKeyCode::Key2, true) => '@',
        (VirtualKeyCode::Key3, false) => '3',
        (VirtualKeyCode::Key3, true) => '#',
        (VirtualKeyCode::Key4, false) => '4',
        (VirtualKeyCode::Key4, true) => '$',
        (VirtualKeyCode::Key5, false) => '5',
        (VirtualKeyCode::Key5, true) => '%',
        (VirtualKeyCode::Key6, false) => '6',
        (VirtualKeyCode::Key6, true) => '^',
        (VirtualKeyCode::Key7, false) => '7',
        (VirtualKeyCode::Key7, true) => '&',
        (VirtualKeyCode::Key8, false) => '8',
        (VirtualKeyCode::Key8, true) => '*',
        (VirtualKeyCode::Key9, false) => '9',
        (VirtualKeyCode::Key9, true) => '(',
        (VirtualKeyCode::Key0, false) => '0',
        (VirtualKeyCode::Key0, true) => ')',
        (VirtualKeyCode::Equals, false) => '=',
        (VirtualKeyCode::Equals, true) => '+',
        (VirtualKeyCode::Minus, false) => '-',
        (VirtualKeyCode::Minus, true) => '_',
        (VirtualKeyCode::RBracket, false) => ']',
        (VirtualKeyCode::RBracket, true) => '}',
        (VirtualKeyCode::LBracket, false) => '[',
        (VirtualKeyCode::LBracket, true) => '{',
        (VirtualKeyCode::Apostrophe, false) => '\'',
        (VirtualKeyCode::Apostrophe, true) => '"',
        (VirtualKeyCode::Semicolon, false) => ';',
        (VirtualKeyCode::Semicolon, true) => ':',
        (VirtualKeyCode::Backslash, false) => '\\',
        (VirtualKeyCode::Backslash, true) => '|',
        (VirtualKeyCode::Comma, false) => ',',
        (VirtualKeyCode::Comma, true) => '<',
        (VirtualKeyCode::Slash, false) => '/',
        (VirtualKeyCode::Slash, true) => '?',
        (VirtualKeyCode::Period, false) => '.',
        (VirtualKeyCode::Period, true) => '>',
        (VirtualKeyCode::Grave, false) => '`',
        (VirtualKeyCode::Grave, true) => '~',
        _ => return None,
    })
}

pub fn scancode_to_keycode(scancode: c_ushort) -> Option<VirtualKeyCode> {
    Some(match scancode {
        0x00 => VirtualKeyCode::A,
        0x01 => VirtualKeyCode::S,
        0x02 => VirtualKeyCode::D,
        0x03 => VirtualKeyCode::F,
        0x04 => VirtualKeyCode::H,
        0x05 => VirtualKeyCode::G,
        0x06 => VirtualKeyCode::Z,
        0x07 => VirtualKeyCode::X,
        0x08 => VirtualKeyCode::C,
        0x09 => VirtualKeyCode::V,
        //0x0a => World 1,
        0x0b => VirtualKeyCode::B,
        0x0c => VirtualKeyCode::Q,
        0x0d => VirtualKeyCode::W,
        0x0e => VirtualKeyCode::E,
        0x0f => VirtualKeyCode::R,
        0x10 => VirtualKeyCode::Y,
        0x11 => VirtualKeyCode::T,
        0x12 => VirtualKeyCode::Key1,
        0x13 => VirtualKeyCode::Key2,
        0x14 => VirtualKeyCode::Key3,
        0x15 => VirtualKeyCode::Key4,
        0x16 => VirtualKeyCode::Key6,
        0x17 => VirtualKeyCode::Key5,
        0x18 => VirtualKeyCode::Equals,
        0x19 => VirtualKeyCode::Key9,
        0x1a => VirtualKeyCode::Key7,
        0x1b => VirtualKeyCode::Minus,
        0x1c => VirtualKeyCode::Key8,
        0x1d => VirtualKeyCode::Key0,
        0x1e => VirtualKeyCode::RBracket,
        0x1f => VirtualKeyCode::O,
        0x20 => VirtualKeyCode::U,
        0x21 => VirtualKeyCode::LBracket,
        0x22 => VirtualKeyCode::I,
        0x23 => VirtualKeyCode::P,
        0x24 => VirtualKeyCode::Return,
        0x25 => VirtualKeyCode::L,
        0x26 => VirtualKeyCode::J,
        0x27 => VirtualKeyCode::Apostrophe,
        0x28 => VirtualKeyCode::K,
        0x29 => VirtualKeyCode::Semicolon,
        0x2a => VirtualKeyCode::Backslash,
        0x2b => VirtualKeyCode::Comma,
        0x2c => VirtualKeyCode::Slash,
        0x2d => VirtualKeyCode::N,
        0x2e => VirtualKeyCode::M,
        0x2f => VirtualKeyCode::Period,
        0x30 => VirtualKeyCode::Tab,
        0x31 => VirtualKeyCode::Space,
        0x32 => VirtualKeyCode::Grave,
        0x33 => VirtualKeyCode::Back,
        //0x34 => unkown,
        0x35 => VirtualKeyCode::Escape,
        0x36 => VirtualKeyCode::RWin,
        0x37 => VirtualKeyCode::LWin,
        0x38 => VirtualKeyCode::LShift,
        //0x39 => Caps lock,
        0x3a => VirtualKeyCode::LAlt,
        0x3b => VirtualKeyCode::LControl,
        0x3c => VirtualKeyCode::RShift,
        0x3d => VirtualKeyCode::RAlt,
        0x3e => VirtualKeyCode::RControl,
        //0x3f => Fn key,
        0x40 => VirtualKeyCode::F17,
        0x41 => VirtualKeyCode::Decimal,
        //0x42 -> unkown,
        0x43 => VirtualKeyCode::Multiply,
        //0x44 => unkown,
        0x45 => VirtualKeyCode::Add,
        //0x46 => unkown,
        0x47 => VirtualKeyCode::Numlock,
        //0x48 => KeypadClear,
        0x49 => VirtualKeyCode::VolumeUp,
        0x4a => VirtualKeyCode::VolumeDown,
        0x4b => VirtualKeyCode::Divide,
        0x4c => VirtualKeyCode::NumpadEnter,
        //0x4d => unkown,
        0x4e => VirtualKeyCode::Subtract,
        0x4f => VirtualKeyCode::F18,
        0x50 => VirtualKeyCode::F19,
        0x51 => VirtualKeyCode::NumpadEquals,
        0x52 => VirtualKeyCode::Numpad0,
        0x53 => VirtualKeyCode::Numpad1,
        0x54 => VirtualKeyCode::Numpad2,
        0x55 => VirtualKeyCode::Numpad3,
        0x56 => VirtualKeyCode::Numpad4,
        0x57 => VirtualKeyCode::Numpad5,
        0x58 => VirtualKeyCode::Numpad6,
        0x59 => VirtualKeyCode::Numpad7,
        0x5a => VirtualKeyCode::F20,
        0x5b => VirtualKeyCode::Numpad8,
        0x5c => VirtualKeyCode::Numpad9,
        0x5d => VirtualKeyCode::Yen,
        //0x5e => JIS Ro,
        //0x5f => unkown,
        0x60 => VirtualKeyCode::F5,
        0x61 => VirtualKeyCode::F6,
        0x62 => VirtualKeyCode::F7,
        0x63 => VirtualKeyCode::F3,
        0x64 => VirtualKeyCode::F8,
        0x65 => VirtualKeyCode::F9,
        //0x66 => JIS Eisuu (macOS),
        0x67 => VirtualKeyCode::F11,
        //0x68 => JIS Kanna (macOS),
        0x69 => VirtualKeyCode::F13,
        0x6a => VirtualKeyCode::F16,
        0x6b => VirtualKeyCode::F14,
        //0x6c => unkown,
        0x6d => VirtualKeyCode::F10,
        //0x6e => unkown,
        0x6f => VirtualKeyCode::F12,
        //0x70 => unkown,
        0x71 => VirtualKeyCode::F15,
        0x72 => VirtualKeyCode::Insert,
        0x73 => VirtualKeyCode::Home,
        0x74 => VirtualKeyCode::PageUp,
        0x75 => VirtualKeyCode::Delete,
        0x76 => VirtualKeyCode::F4,
        0x77 => VirtualKeyCode::End,
        0x78 => VirtualKeyCode::F2,
        0x79 => VirtualKeyCode::PageDown,
        0x7a => VirtualKeyCode::F1,
        0x7b => VirtualKeyCode::Left,
        0x7c => VirtualKeyCode::Right,
        0x7d => VirtualKeyCode::Down,
        0x7e => VirtualKeyCode::Up,
        //0x7f =>  unkown,
        0xa => VirtualKeyCode::Caret,
        _ => return None,
    })
}

// While F1-F20 have scancodes we can match on, we have to check against UTF-16
// constants for the rest.
// https://developer.apple.com/documentation/appkit/1535851-function-key_unicodes?preferredLanguage=occ
pub fn check_function_keys(string: &str) -> Option<VirtualKeyCode> {
    if let Some(ch) = string.encode_utf16().next() {
        return Some(match ch {
            0xf718 => VirtualKeyCode::F21,
            0xf719 => VirtualKeyCode::F22,
            0xf71a => VirtualKeyCode::F23,
            0xf71b => VirtualKeyCode::F24,
            _ => return None,
        });
    }

    None
}

pub fn event_mods(event: id) -> ModifiersState {
    let flags = unsafe { NSEvent::modifierFlags(event) };
    let mut m = ModifiersState::empty();
    m.set(
        ModifiersState::SHIFT,
        flags.contains(NSEventModifierFlags::NSShiftKeyMask),
    );
    m.set(
        ModifiersState::CTRL,
        flags.contains(NSEventModifierFlags::NSControlKeyMask),
    );
    m.set(
        ModifiersState::ALT,
        flags.contains(NSEventModifierFlags::NSAlternateKeyMask),
    );
    m.set(
        ModifiersState::LOGO,
        flags.contains(NSEventModifierFlags::NSCommandKeyMask),
    );
    m
}

pub fn get_scancode(event: cocoa::base::id) -> c_ushort {
    // In AppKit, `keyCode` refers to the position (scancode) of a key rather than its character,
    // and there is no easy way to navtively retrieve the layout-dependent character.
    // In winit, we use keycode to refer to the key's character, and so this function aligns
    // AppKit's terminology with ours.
    unsafe { msg_send![event, keyCode] }
}

pub unsafe fn modifier_event(
    ns_event: id,
    keymask: NSEventModifierFlags,
    was_key_pressed: bool,
) -> Option<WindowEvent<'static>> {
    if !was_key_pressed && NSEvent::modifierFlags(ns_event).contains(keymask)
        || was_key_pressed && !NSEvent::modifierFlags(ns_event).contains(keymask)
    {
        let state = if was_key_pressed {
            ElementState::Released
        } else {
            ElementState::Pressed
        };

        let scancode = get_scancode(ns_event);
        let virtual_keycode = scancode_to_keycode(scancode);
        #[allow(deprecated)]
        Some(WindowEvent::KeyboardInput {
            device_id: DEVICE_ID,
            input: KeyboardInput {
                state,
                scancode: scancode as _,
                virtual_keycode,
                modifiers: event_mods(ns_event),
            },
            is_synthetic: false,
        })
    } else {
        None
    }
}
