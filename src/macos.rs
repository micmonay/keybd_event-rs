#![allow(non_upper_case_globals)]
use core_graphics::event::{CGEvent, CGEventFlags, CGEventTapLocation};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
use std::thread::sleep;
use std::time::Duration;
use KeyboardKey;
use {KBPlatform, KeyBonding};

pub struct MacOSKeyBD {
    special_flags: CGEventFlags,
}

impl KBPlatform for MacOSKeyBD {
    fn run_action(&mut self, key_bonding: KeyBonding) {
        key_bonding.keys.iter().for_each(|keycode| {
            self.key_press(keycode, &key_bonding).err();
        });
    }
}

impl MacOSKeyBD {
    pub fn new() -> Result<Box<KBPlatform>, String> {
        return Ok(Box::new(MacOSKeyBD {
            special_flags: CGEventFlags::CGEventFlagNull,
        }));
    }
    fn key_press(&self, key_code: &KeyboardKey, key_bonding: &KeyBonding) -> Result<bool, String> {
        let event_source_down = match CGEventSource::new(CGEventSourceStateID::CombinedSessionState)
        {
            Ok(v) => v,
            Err(_) => return Err("not succes creating keyboard event".to_string()),
        };
        let event_source_up = match CGEventSource::new(CGEventSourceStateID::CombinedSessionState) {
            Ok(v) => v,
            Err(_) => return Err("not succes creating keyboard event".to_string()),
        };
        let key_code_macos = match MacOSKeyBD::convert_keycode(key_code) {
            None => return Err("Not supported keys".to_string()),
            Some(key) => key,
        };
        let event_down = match CGEvent::new_keyboard_event(event_source_down, key_code_macos, true)
        {
            Ok(event) => event,
            Err(_) => return Err("not succes creating keyboard event".to_string()),
        };
        let event_up = match CGEvent::new_keyboard_event(event_source_up, key_code_macos, false) {
            Ok(event) => event,
            Err(_) => return Err("not succes creating keyboard event".to_string()),
        };
        let mut flags = CGEventFlags::CGEventFlagNull;
        if key_bonding.has_shift {
            flags |= CGEventFlags::CGEventFlagShift;
        }
        if key_bonding.has_altgr || key_bonding.has_alt {
            flags |= CGEventFlags::CGEventFlagAlternate;
        }
        if key_bonding.has_rctrl || key_bonding.has_ctrl {
            flags |= CGEventFlags::CGEventFlagControl;
        }
        if self.special_flags != CGEventFlags::CGEventFlagNull {
            flags |= self.special_flags;
        }
        event_down.set_flags(flags as CGEventFlags);
        event_up.set_flags(flags as CGEventFlags);

        event_down.post(CGEventTapLocation::AnnotatedSession);
        sleep(Duration::from_millis(10));
        event_up.post(CGEventTapLocation::AnnotatedSession);
        return Ok(true);
    }

    fn convert_keycode(keycode: &KeyboardKey) -> Option<u16> {
        return match keycode {
            KeyboardKey::KeySP1 => Some(0x0A),
            KeyboardKey::KeySP2 => Some(0x1B),
            KeyboardKey::KeySP3 => Some(0x18),
            KeyboardKey::KeySP4 => Some(0x21),
            KeyboardKey::KeySP5 => Some(0x1E),
            KeyboardKey::KeySP6 => Some(0x29),
            KeyboardKey::KeySP7 => Some(0x27),
            KeyboardKey::KeySP8 => Some(0x2A),
            KeyboardKey::KeySP9 => Some(0x2B),
            KeyboardKey::KeySP10 => Some(0x2F),
            KeyboardKey::KeySP11 => Some(0x2C),
            KeyboardKey::KeySP12 => Some(0x32),
            KeyboardKey::KeyUP => Some(0x7E),
            KeyboardKey::KeyDOWN => Some(0x7D),
            KeyboardKey::KeyLEFT => Some(0x7B),
            KeyboardKey::KeyRIGHT => Some(0x7C),
            KeyboardKey::KeyESC => Some(0x35),
            KeyboardKey::Key1 => Some(0x12),
            KeyboardKey::Key2 => Some(0x13),
            KeyboardKey::Key3 => Some(0x14),
            KeyboardKey::Key4 => Some(0x15),
            KeyboardKey::Key5 => Some(0x17),
            KeyboardKey::Key6 => Some(0x16),
            KeyboardKey::Key7 => Some(0x1A),
            KeyboardKey::Key8 => Some(0x1C),
            KeyboardKey::Key9 => Some(0x19),
            KeyboardKey::Key0 => Some(0x1D),
            KeyboardKey::KeyQ => Some(0x0C),
            KeyboardKey::KeyW => Some(0x0D),
            KeyboardKey::KeyE => Some(0x0E),
            KeyboardKey::KeyR => Some(0x0F),
            KeyboardKey::KeyT => Some(0x11),
            KeyboardKey::KeyY => Some(0x10),
            KeyboardKey::KeyU => Some(0x20),
            KeyboardKey::KeyI => Some(0x22),
            KeyboardKey::KeyO => Some(0x1F),
            KeyboardKey::KeyP => Some(0x23),
            KeyboardKey::KeyA => Some(0x00),
            KeyboardKey::KeyS => Some(0x01),
            KeyboardKey::KeyD => Some(0x02),
            KeyboardKey::KeyF => Some(0x03),
            KeyboardKey::KeyG => Some(0x05),
            KeyboardKey::KeyH => Some(0x04),
            KeyboardKey::KeyJ => Some(0x26),
            KeyboardKey::KeyK => Some(0x28),
            KeyboardKey::KeyL => Some(0x25),
            KeyboardKey::KeyZ => Some(0x06),
            KeyboardKey::KeyX => Some(0x07),
            KeyboardKey::KeyC => Some(0x08),
            KeyboardKey::KeyV => Some(0x09),
            KeyboardKey::KeyB => Some(0x0B),
            KeyboardKey::KeyN => Some(0x2D),
            KeyboardKey::KeyM => Some(0x2E),
            KeyboardKey::KeyF1 => Some(0x7A),
            KeyboardKey::KeyF2 => Some(0x78),
            KeyboardKey::KeyF3 => Some(0x63),
            KeyboardKey::KeyF4 => Some(0x76),
            KeyboardKey::KeyF5 => Some(0x60),
            KeyboardKey::KeyF6 => Some(0x61),
            KeyboardKey::KeyF7 => Some(0x62),
            KeyboardKey::KeyF8 => Some(0x64),
            KeyboardKey::KeyF9 => Some(0x65),
            KeyboardKey::KeyF10 => Some(0x6D),
            KeyboardKey::KeyF11 => Some(0x67),
            KeyboardKey::KeyF12 => Some(0x6F),
            KeyboardKey::KeyNUMLock => None,
            KeyboardKey::KeyScrollLock => None,
            KeyboardKey::KeyRESERVED => None,
            KeyboardKey::KeyBACKSPACE => Some(0x33),
            KeyboardKey::KeyTAB => Some(0x30),
            KeyboardKey::KeyENTER => Some(0x24),
            KeyboardKey::KeySPACE => Some(0x31),
            KeyboardKey::KeyCAPSLock => Some(0x39),
            KeyboardKey::KeyKP0 => Some(0x52),
            KeyboardKey::KeyKP1 => Some(0x53),
            KeyboardKey::KeyKP2 => Some(0x54),
            KeyboardKey::KeyKP3 => Some(0x55),
            KeyboardKey::KeyKP4 => Some(0x56),
            KeyboardKey::KeyKP5 => Some(0x57),
            KeyboardKey::KeyKP6 => Some(0x58),
            KeyboardKey::KeyKP7 => Some(0x59),
            KeyboardKey::KeyKP8 => Some(0x5B),
            KeyboardKey::KeyKP9 => Some(0x5C),
            KeyboardKey::KeyKPMinus => Some(0x4E),
            KeyboardKey::KeyKPPlus => Some(0x45),
            KeyboardKey::KeyKPDot => Some(0x41),
            KeyboardKey::KeyKPJPComma => None,
            KeyboardKey::KeyKPEnter => Some(0x4C),
            KeyboardKey::KeyKPSlash => Some(0x4B),
            KeyboardKey::KeyKPAsterisk => Some(0x43),
            KeyboardKey::KeyKPEqual => Some(0x51),
            KeyboardKey::KeyKPPlusMinus => None,
            KeyboardKey::KeyKPComma => None,
        };
    }
}
