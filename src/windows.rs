use std::thread::sleep;
use std::time::Duration;
use {KBPlatform, KeyBonding};

#[link(name = "user32")]
extern "C" {
    fn keybd_event(b_vk: u8, b_scan: u8, dw_flags: u16, dw_extra_info: usize);
}

const FLAG_KEYUP: u16 = 0x0002;
const FLAG_SCAN_CODE: u16 = 0x0008;
// I add 0xFFF for because is virtual key
const K_SHIFT: u16 = 0x10 + 0xFFF;
const K_CTRL: u16 = 0x11 + 0xFFF;
const K_ALT: u16 = 0x12 + 0xFFF;
const K_RSHIFT: u16 = 0xA1 + 0xFFF;
// not use
// const K_LSHIFT: u16 = 0xA0 + 0xFFF;
// const K_LCONTROL: u16 = 0xA2 + 0xFFF;
const K_RCONTROL: u16 = 0xA3 + 0xFFF;
pub struct WindowsKeyBD {}
impl KBPlatform for WindowsKeyBD {
    fn run_action(&mut self, key_bonding: KeyBonding) {
        unsafe {
            if key_bonding.has_alt {
                self.down_key(K_ALT);
            }
            if key_bonding.has_altgr {
                self.down_key(K_ALT);
                self.down_key(K_CTRL);
            }
            if key_bonding.has_shift {
                self.down_key(K_SHIFT);
            }
            if key_bonding.has_ctrl {
                self.down_key(K_CTRL);
            }
            if key_bonding.has_rshift {
                self.down_key(K_RSHIFT);
            }
            if key_bonding.has_rctrl {
                self.down_key(K_RCONTROL);
            }
            key_bonding.keys.iter().for_each(|value| {
                self.down_key(*value as u16);
                self.up_key(*value as u16);
            });
            if key_bonding.has_alt {
                self.up_key(K_ALT);
            }
            if key_bonding.has_altgr {
                self.up_key(K_ALT);
                self.up_key(K_CTRL);
            }
            if key_bonding.has_shift {
                self.up_key(K_SHIFT);
            }
            if key_bonding.has_ctrl {
                self.up_key(K_CTRL);
            }
            if key_bonding.has_rshift {
                self.up_key(K_RSHIFT);
            }
            if key_bonding.has_rctrl {
                self.up_key(K_RCONTROL);
            }
        }
    }
}
impl WindowsKeyBD {
    pub fn new() -> Result<Box<KBPlatform>, String> {
        Ok(Box::new(WindowsKeyBD {}))
    }
    unsafe fn down_key(&self, mut key: u16) {
        let mut flag = 0;
        if key < 0xFFF {
            // Detect if the key code is no virtual
            flag |= FLAG_SCAN_CODE;
        } else {
            key -= 0xFFF;
        }
        let v_key = key + 0x80;
        keybd_event(key as u8, v_key as u8, flag, 0);
    }

    unsafe fn up_key(&self, mut key: u16) {
        let mut flag = FLAG_KEYUP;
        if key < 0xFFF {
            // Detect if the key code is no virtual
            flag |= FLAG_SCAN_CODE;
        } else {
            key -= 0xFFF;
        }
        let v_key = key + 0x80;
        keybd_event(key as u8, v_key as u8, flag, 0);
    }
}
