use std::fs::File;
use std::io::ErrorKind;
use std::path::Path;

use uinput::Device;

use {KBPlatform, KeyBonding};

const EV_SYN: i32 = 0x00;
const EV_KEY: i32 = 0x01;

const K_LEFT_CTRL: u8 = 29;
const K_RIGHT_CTRL: u8 = 97;
const K_CTRL: u8 = 29;
const K_LEFT_SHIFT: u8 = 42;
const K_RIGHT_SHIFT: u8 = 54;
const K_SHIFT: u8 = 42;
const K_LEFT_ALT: u8 = 56;
const K_RIGHT_ALT: u8 = 100;
const K_ALT: u8 = 56;

pub struct LinuxKeyBD {
    instance: Device,
}

impl KBPlatform for LinuxKeyBD {
    fn run_action(&mut self, key_bonding: KeyBonding) {
        if key_bonding.has_alt {
            self.down_key(K_ALT);
        }
        if key_bonding.has_altgr {
            self.down_key(K_RIGHT_ALT);
        }
        if key_bonding.has_shift {
            self.down_key(K_SHIFT);
        }
        if key_bonding.has_ctrl {
            self.down_key(K_CTRL);
        }
        if key_bonding.has_rshift {
            self.down_key(K_RIGHT_SHIFT);
        }
        if key_bonding.has_rctrl {
            self.down_key(K_RIGHT_CTRL);
        }
        key_bonding.keys.iter().for_each(|value| {
            self.down_key(*value as u8);
        });
        key_bonding.keys.iter().for_each(|value| {
            self.up_key(*value as u8);
        });
        if key_bonding.has_alt {
            self.up_key(K_ALT);
        }
        if key_bonding.has_altgr {
            self.up_key(K_RIGHT_ALT);
        }
        if key_bonding.has_shift {
            self.up_key(K_SHIFT);
        }
        if key_bonding.has_ctrl {
            self.up_key(K_CTRL);
        }
        if key_bonding.has_rshift {
            self.up_key(K_RIGHT_SHIFT);
        }
        if key_bonding.has_rctrl {
            self.up_key(K_RIGHT_CTRL);
        }
        self.instance.synchronize().unwrap();
    }
}

impl LinuxKeyBD {
    pub fn new() -> Result<Box<KBPlatform>, String> {
        let path_uinput = match get_path_uinput() {
            Ok(path) => path,
            Err(error) => return Err(error.to_string()),
        };
        match File::open(path_uinput) {
            Ok(file) => file,
            Err(error) => {
                if error.kind() == ErrorKind::PermissionDenied {
                    let path = path_uinput.to_str().unwrap_or("PATH_ERROR");
                    return Err(format!(
                        "permission error for {} try cmd : sudo chmod +0666 {}",
                        path, path
                    ));
                }
                return Err(error.to_string());
            }
        };
        let mut builder = match uinput::open(path_uinput) {
            Ok(builder) => builder,
            Err(error) => return Err(error.to_string()),
        };
        builder = match builder.name("keybd_event") {
            Ok(builder) => builder,
            Err(error) => return Err(error.to_string()),
        };
        builder = match builder.event(uinput::event::Keyboard::All) {
            Ok(builder) => builder,
            Err(error) => return Err(error.to_string()),
        };
        return match builder.create() {
            Ok(device) => Ok(Box::new(LinuxKeyBD { instance: device })),
            Err(error) => Err(error.to_string()),
        };
    }
    fn down_key(&mut self, key: u8) -> bool {
        return self.instance.write(EV_KEY, key as i32, 1).is_ok();
    }
    fn up_key(&mut self, key: u8) -> bool {
        return self.instance.write(EV_KEY, key as i32, 0).is_ok();
    }
}

fn get_path_uinput<'a>() -> Result<&'a Path, String> {
    if Path::new("/dev/uinput").exists() {
        return Ok(Path::new("/dev/uinput"));
    }
    if Path::new("/dev/input/uinput").exists() {
        return Ok(Path::new("/dev/input/uinput"));
    }
    return Err("Not found uinput file. Try this cmd 'sudo modprobe uinput'".to_string());
}
