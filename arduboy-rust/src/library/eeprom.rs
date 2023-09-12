use core::ffi::{c_int, c_uchar};

pub const EEPROM_STORAGE_SPACE_START: i16 = 16;

extern "C" {
    #[link_name = "arduboy_eeprom_read"]
    fn arduboy_eeprom_read_raw(idx: c_int) -> c_uchar;
    #[link_name = "arduboy_eeprom_update"]
    fn arduboy_eeprom_update_raw(idx: c_int, val: c_uchar);
    #[link_name = "arduboy_eeprom_write"]
    fn arduboy_eeprom_write_raw(idx: c_int, val: c_uchar);
    #[link_name = "arduboy_eeprom_get"]
    fn arduboy_eeprom_get_raw(idx: c_int, object: *mut u8, size: usize);
    #[link_name = "arduboy_eeprom_put"]
    fn arduboy_eeprom_put_raw(idx: c_int, object: *const u8, size: usize);
}
///This is the struct to store and read structs objects to/from eeprom memory.
/// ## Example
/// ```
/// static e: EEPROM = EEPROM::new(10);
/// struct Scorebord {
///     player1: u16,
///     text: &'static str,
/// }
/// static mut s: Scorebord = Scorebord {
///     player1: 0,
///     text: "lol\0",
/// };
///
/// // init inside of the setup function
/// e.init(&mut s);
/// ```
pub struct EEPROM {
    start_c1: i16,
    start_c2: i16,
    idx: i16,
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
    pub fn init<T>(&self, your_struct: &mut T) {
        let c1 = self.read(self.start_c1);
        let c2 = self.read(self.start_c2);

        if c1 != b'Z' || c2 != b'D' {
            self.update(self.start_c1, b'Z');
            self.update(self.start_c2, b'D');
            self.put(your_struct);
        };
        self.get(your_struct)
    }
    fn read(&self, idx: i16) -> u8 {
        unsafe { arduboy_eeprom_read_raw(idx) }
    }
    fn update(&self, idx: i16, val: u8) {
        unsafe { arduboy_eeprom_update_raw(idx, val) }
    }
    pub fn get<T>(&self, your_struct: &mut T) {
        let pointer = your_struct as *mut T;
        let object_pointer = pointer as *mut u8;
        let object_size = core::mem::size_of::<T>();

        unsafe {
            arduboy_eeprom_get_raw(self.idx, object_pointer, object_size);
        };
    }
    pub fn get_direct<T>(&self) -> T {
        let mut buffer = core::mem::MaybeUninit::<T>::uninit();

        let pointer = buffer.as_mut_ptr();
        let object_pointer = pointer as *mut u8;
        let object_size = core::mem::size_of::<T>();

        unsafe {
            arduboy_eeprom_get_raw(self.idx, object_pointer, object_size);
        };

        return unsafe { buffer.assume_init() };
    }
    pub fn put<T>(&self, your_struct: &T) {
        let pointer = your_struct as *const T;
        let object_pointer = pointer as *const u8;
        let object_size = core::mem::size_of::<T>();

        unsafe {
            arduboy_eeprom_put_raw(self.idx, object_pointer, object_size);
        };
    }
}
///Use this struct to store and read single bytes to/from eeprom memory.
pub struct EEPROMBYTE {
    start_c1: i16,
    start_c2: i16,
    idx: i16,
}
impl EEPROMBYTE {
    pub const fn new(mut idx: i16) -> EEPROMBYTE {
        if idx > 1010 {
            idx = 0
        }
        EEPROMBYTE {
            start_c1: EEPROM_STORAGE_SPACE_START + idx,
            start_c2: EEPROM_STORAGE_SPACE_START + idx + 1,
            idx: EEPROM_STORAGE_SPACE_START + idx + 2,
        }
    }
    pub fn init(&self) {
        let c1 = self.read_intern(self.start_c1);
        let c2 = self.read_intern(self.start_c2);

        if c1 != b'Z' || c2 != b'D' {
            self.update_intern(self.start_c1, b'Z');
            self.update_intern(self.start_c2, b'D');
            self.update(0);
        };
    }
    fn read_intern(&self, idx: i16) -> u8 {
        unsafe { arduboy_eeprom_read_raw(idx) }
    }
    pub fn read(&self) -> u8 {
        unsafe { arduboy_eeprom_read_raw(self.idx) }
    }
    fn update_intern(&self, idx: i16, val: u8) {
        unsafe { arduboy_eeprom_update_raw(idx, val) }
    }
    pub fn update(&self, val: u8) {
        unsafe { arduboy_eeprom_update_raw(self.idx, val) }
    }
    pub fn write(&self, val: u8) {
        unsafe { arduboy_eeprom_write_raw(self.idx, val) }
    }
}

///Use this struct to store and read single bytes to/from eeprom memory without using a check digit.
///Unlike the other eeprom structs, this does not need to be initialised.
pub struct EEPROMBYTECHECKLESS {
    idx: i16,
}
impl EEPROMBYTECHECKLESS {
    pub const fn new(mut idx: i16) -> EEPROMBYTECHECKLESS {
        if idx > 1010 {
            idx = 0
        }
        EEPROMBYTECHECKLESS {
            idx: EEPROM_STORAGE_SPACE_START + idx,
        }
    }
    pub fn read(&self) -> u8 {
        unsafe { arduboy_eeprom_read_raw(self.idx) }
    }
    pub fn update(&self, val: u8) {
        unsafe { arduboy_eeprom_update_raw(self.idx, val) }
    }
    pub fn write(&self, val: u8) {
        unsafe { arduboy_eeprom_write_raw(self.idx, val) }
    }
}
