//! # Simulating keyboard on Linux, Windows and Mac OS in rust
//!
//! On the next example, the library simulates key A, Z pressed.
//!
//! **The keyboard layout on the computer is important!**
//!
//! If you use a keyboard layout the US, you have corresponding keys, but if you use, for example, the french layout, you have another result.
//! ```
//!extern crate keybd_event;
//!
//!#[cfg(target_os = "linux")]
//!use std::thread::sleep;
//!#[cfg(target_os = "linux")]
//!use std::time::Duration;
//!use keybd_event::KeyboardKey::{KeyA,KeyZ};
//!use keybd_event::KeyBondingInstance;
//!
//!fn main() {
//!    let mut kb = KeyBondingInstance::new().unwrap();
//!    #[cfg(target_os = "linux")]
//!        sleep(Duration::from_secs(2));
//!    kb.has_shift(true);
//!    kb.add_keys(&[KeyA, KeyZ]);
//!    kb.launching();
//!}
//! ```
//! <div style="text-align: center;"><img alt="keyboard image" src="https://github.com/micmonay/keybd_event-rs/raw/master/keyboard-rust.png"/></div>
//!
//! ## Linux
//!
//! On Linux this library use **uinput**, but generally the uinput is only for the root user.
//!
//! The easy solution is executing on root user or change permission by `chmod`, but it is not good.
//!
//! You can follow the next example, for more security.
//!
//!```bash
//!sudo groupadd uinput
//!sudo usermod -a -G uinput my_username
//!sudo udevadm control --reload-rules
//!echo "SUBSYSTEM==\"misc\", KERNEL==\"uinput\", GROUP=\"uinput\", MODE=\"0660\"" | sudo tee /etc/udev/rules.d/uinput.rules
//!echo uinput | sudo tee /etc/modules-load.d/uinput.conf
//!```
//!
//! Another subtlety on Linux, it is important after creating **KeyBondingInstance**, to waiting 2 seconds before running first keyboard actions
//!
//! ## Darwin (MAC OS)
//! This library depends on the frameworks Apple, I did not find a solution for cross-compilation.
#[cfg(target_os = "macos")]
extern crate core_graphics;
#[cfg(target_os = "linux")]
extern crate uinput;

#[cfg(target_os = "linux")]
use linux::LinuxKeyBD;
#[cfg(target_os = "macos")]
use macos::MacOSKeyBD;
#[cfg(target_os = "windows")]
use windows::WindowsKeyBD;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;

/// Contain all Keyboard key compatible
#[derive(Copy, Clone, Debug)]
pub enum KeyboardKey {
    KeySP1 = 41,
    KeySP2 = 12,
    KeySP3 = 13,
    KeySP4 = 26,
    KeySP5 = 27,
    KeySP6 = 39,
    KeySP7 = 40,
    KeySP8 = 43,
    KeySP9 = 51,
    KeySP10 = 52,
    KeySP11 = 53,
    KeySP12 = 86,
    KeyUP = 103,
    KeyDOWN = 108,
    KeyLEFT = 105,
    KeyRIGHT = 106,
    KeyESC = 1,
    Key1 = 2,
    Key2 = 3,
    Key3 = 4,
    Key4 = 5,
    Key5 = 6,
    Key6 = 7,
    Key7 = 8,
    Key8 = 9,
    Key9 = 10,
    Key0 = 11,
    KeyQ = 16,
    KeyW = 17,
    KeyE = 18,
    KeyR = 19,
    KeyT = 20,
    KeyY = 21,
    KeyU = 22,
    KeyI = 23,
    KeyO = 24,
    KeyP = 25,
    KeyA = 30,
    KeyS = 31,
    KeyD = 32,
    KeyF = 33,
    KeyG = 34,
    KeyH = 35,
    KeyJ = 36,
    KeyK = 37,
    KeyL = 38,
    KeyZ = 44,
    KeyX = 45,
    KeyC = 46,
    KeyV = 47,
    KeyB = 48,
    KeyN = 49,
    KeyM = 50,
    KeyF1 = 59,
    KeyF2 = 60,
    KeyF3 = 61,
    KeyF4 = 62,
    KeyF5 = 63,
    KeyF6 = 64,
    KeyF7 = 65,
    KeyF8 = 66,
    KeyF9 = 67,
    KeyF10 = 68,
    KeyF11 = 87,
    KeyF12 = 88,
    KeyNUMLock = 69,
    KeyScrollLock = 70,
    KeyRESERVED = 0,
    KeyBACKSPACE = 14,
    KeyTAB = 15,
    KeyENTER = 28,
    KeySPACE = 57,
    KeyCAPSLock = 58,
    KeyKP0 = 82,
    KeyKP1 = 79,
    KeyKP2 = 80,
    KeyKP3 = 81,
    KeyKP4 = 75,
    KeyKP5 = 76,
    KeyKP6 = 77,
    KeyKP7 = 71,
    KeyKP8 = 72,
    KeyKP9 = 73,
    KeyKPMinus = 74,
    KeyKPPlus = 78,
    KeyKPDot = 83,
    KeyKPJPComma = 95,
    KeyKPEnter = 96,
    KeyKPSlash = 98,
    KeyKPAsterisk = 55,
    KeyKPEqual = 117,
    KeyKPPlusMinus = 118,
    KeyKPComma = 121,
}

/// All platform need implement this trait.
pub trait KBPlatform {
    fn run_action(&mut self, key_bonding: KeyBonding);
}

/// Use for create and run the simulation.
pub struct KeyBondingInstance {
    key_bonding: KeyBonding,
    platform: Box<dyn KBPlatform>,
}

/// Data information for platform.
#[derive(Clone, Debug)]
pub struct KeyBonding {
    pub has_ctrl: bool,
    pub has_alt: bool,
    pub has_shift: bool,
    pub has_rctrl: bool,
    pub has_rshift: bool,
    pub has_altgr: bool,
    pub keys: Vec<KeyboardKey>,
}

impl KeyBondingInstance {
    /// Default function for create a new instance of KeyBondingInstance.
    pub fn new() -> Result<KeyBondingInstance, String> {
        let platform = match KeyBondingInstance::get_platform() {
            Ok(p) => p,
            Err(error) => return Err(error),
        };
        return KeyBondingInstance::new_with_platform(platform);
    }
    /// For create new KeyBondingInstance with specific platform.
    pub fn new_with_platform(platform: Box<dyn KBPlatform>) -> Result<KeyBondingInstance, String> {
        Ok(KeyBondingInstance {
            key_bonding: KeyBonding {
                has_ctrl: false,
                has_alt: false,
                has_shift: false,
                has_rctrl: false,
                has_rshift: false,
                has_altgr: false,
                keys: vec![],
            },
            platform,
        })
    }
    fn get_platform() -> Result<Box<dyn KBPlatform>, String> {
        #[cfg(target_os = "windows")]
        return WindowsKeyBD::new();
        #[cfg(target_os = "linux")]
        return LinuxKeyBD::new();
        #[cfg(target_os = "macos")]
        return MacOSKeyBD::new();

        #[allow(unreachable_code)]
        Err("Not compatible platform for keybd_event".to_string())
    }
    /// Clean data of KeyBonding
    pub fn clear(&mut self) {
        self.key_bonding = KeyBonding {
            has_ctrl: false,
            has_alt: false,
            has_shift: false,
            has_rctrl: false,
            has_rshift: false,
            has_altgr: false,
            keys: vec![],
        }
    }

    pub fn set_keys(&mut self, keys: Vec<KeyboardKey>) {
        self.key_bonding.keys = keys;
    }

    pub fn add_keys(&mut self, keys: &[KeyboardKey]) {
        self.key_bonding.keys.extend_from_slice(keys);
    }
    pub fn add_key(&mut self, key: KeyboardKey) {
        self.key_bonding.keys.push(key);
    }
    pub fn has_shift(&mut self, b: bool) {
        self.key_bonding.has_shift = b;
    }
    pub fn has_alt(&mut self, b: bool) {
        self.key_bonding.has_alt = b;
    }
    pub fn has_ctrl(&mut self, b: bool) {
        self.key_bonding.has_ctrl = b;
    }
    pub fn has_rctrl(&mut self, b: bool) {
        self.key_bonding.has_rctrl = b;
    }
    pub fn has_rshift(&mut self, b: bool) {
        self.key_bonding.has_rshift = b;
    }

    pub fn has_altgr(&mut self, b: bool) {
        self.key_bonding.has_altgr = b;
    }
    /// For launch the simulation
    pub fn launching(&mut self) {
        self.platform.run_action(self.key_bonding.clone());
    }
}

#[cfg(test)]
mod tests {
    #[cfg(target_os = "linux")]
    use std::thread::{sleep, Thread};
    #[cfg(target_os = "linux")]
    use std::time::Duration;

    use KeyBondingInstance;
    use KeyboardKey::*;

    #[test]
    fn it_works() {
        let mut kb = KeyBondingInstance::new().unwrap();
        #[cfg(target_os = "linux")]
        sleep(Duration::from_secs(2));
        kb.has_shift(true);
        kb.add_keys(&[KeyA, KeyZ]);
        kb.launching();
    }
}
