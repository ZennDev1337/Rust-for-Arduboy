use core::ffi::{c_int, c_uchar};

pub const EEPROM_STORAGE_SPACE_START: c_int = 16;

extern "C" {
    #[link_name = "arduboy_eeprom_read"]
    fn arduboy_eeprom_read_raw(idx: c_int) -> c_uchar;
    #[link_name = "arduboy_eeprom_update"]
    fn arduboy_eeprom_update_raw(idx: c_int, val: c_uchar);
    #[link_name = "arduboy_eeprom_write"]
    fn arduboy_eeprom_write_raw(idx: c_int, val: c_uchar);
    #[link_name = "arduboy_eeprom_get"]
    fn arduboy_eeprom_get_raw(idx: c_int) -> c_uchar;
    #[link_name = "arduboy_eeprom_put"]
    fn arduboy_eeprom_put_raw(idx: c_int, val: c_uchar);
}
///This is the struct to interact in a save way with the EEPROM C++ library.
pub struct EEPROM {
    start_c1: i16,
    start_c2: i16,
    pub idx: i16,
}
impl EEPROM {
    pub const fn new(mut idx: i16) -> EEPROM {
        if idx > 950 {
            idx = 0
        }
        EEPROM {
            start_c1: EEPROM_STORAGE_SPACE_START + idx,
            start_c2: EEPROM_STORAGE_SPACE_START + idx + 1,
            idx: EEPROM_STORAGE_SPACE_START + idx + 2,
        }
    }
    pub fn init(&self) {
        let c1 = self.read_start_c1();
        let c2 = self.read_start_c2();

        if c1 != b'Z' || c2 != b'D' {
            self.update_start_c1(b'Z');
            self.update_start_c2(b'D');
            self.update(0);
        };
    }
    pub fn read(&self) -> u8 {
        unsafe { arduboy_eeprom_read_raw(self.idx) }
    }
    pub fn read_start_c1(&self) -> u8 {
        unsafe { arduboy_eeprom_read_raw(self.start_c1) }
    }
    pub fn read_start_c2(&self) -> u8 {
        unsafe { arduboy_eeprom_read_raw(self.start_c2) }
    }
    pub fn update(&self, val: u8) {
        unsafe { arduboy_eeprom_update_raw(self.idx, val) }
    }
    pub fn update_start_c1(&self, val: u8) {
        unsafe { arduboy_eeprom_update_raw(self.start_c1, val) }
    }
    pub fn update_start_c2(&self, val: u8) {
        unsafe { arduboy_eeprom_update_raw(self.start_c2, val) }
    }
    pub fn write(&self, val: u8) {
        unsafe { arduboy_eeprom_write_raw(self.idx, val) }
    }
    pub fn get(&self, buffer: &mut u8) {
        *buffer = unsafe { arduboy_eeprom_get_raw(self.idx) }
    }
    pub fn put(&self, val: u8) {
        unsafe { arduboy_eeprom_put_raw(self.idx, val) }
    }
}
