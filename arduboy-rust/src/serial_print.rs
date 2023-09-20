//! This is the Module to interact in a save way with the Arduino Serial C++ library.
//!
//! You will need to uncomment the Arduino_Serial_Library in the import_config.h file.
use crate::prelude::Pstring;
use core::ffi::{c_char, c_int, c_long, c_size_t, c_uchar, c_uint, c_ulong, CStr};

use crate::print::Base;
extern "C" {
    #[link_name = "arduino_serial_begin"]
    fn serial_begin(serial: c_ulong);
    #[link_name = "arduino_serial_end"]
    fn serial_end();
    #[link_name = "arduino_serial_available"]
    fn serial_available() -> c_int;
    #[link_name = "arduino_serial_read"]
    fn serial_read() -> c_int;
    #[link_name = "arduino_serial_read_string"]
    fn serial_read_string() -> CStr;
}

///The Arduino Serial Print class is available for writing text to the screen buffer.
///
///In the same manner as the Arduino arduboy.print(), etc., functions.
///
///
///Example
/// ```
/// let value: i16 = 42;
///
/// serial::print(b"Hello World\n\0"[..]); // Prints "Hello World" and then sets the
///                                        // text cursor to the start of the next line
/// serial::print(f!(b"Hello World\n")); // Prints "Hello World" but does not use the 2kb ram
/// serial::print(value); // Prints "42"
/// serial::print("\n\0"); // Sets the text cursor to the start of the next line
/// serial::print("hello world") // Prints normal [&str]
/// ```
pub fn print(x: impl Serialprintable) {
    x.print()
}
///The Arduino Serial Print class is available for writing text to the screen buffer.
///
///In the same manner as the Arduino arduboy.print(), etc., functions.
///
///
///Example
/// ```
/// let value: i16 = 42;
///
/// serial::print(b"Hello World\n\0"[..]); // Prints "Hello World" and then sets the
///                                        // text cursor to the start of the next line
/// serial::print(f!(b"Hello World\n")); // Prints "Hello World" but does not use the 2kb ram
/// serial::print(value); // Prints "42"
/// serial::print("\n\0"); // Sets the text cursor to the start of the next line
/// serial::print("hello world") // Prints normal [&str]
/// ```
pub fn println(x: impl Serialprintlnable) {
    x.println()
}
/// Sets the data rate in bits per second (baud) for serial data transmission. For communicating with Serial Monitor, make sure to use one of the baud rates listed in the menu at the bottom right corner of its screen. You can, however, specify other rates - for example, to communicate over pins 0 and 1 with a component that requires a particular baud rate.
///
/// ### Example
/// ```
/// serial::begin(9600)
/// ```
pub fn begin(baud_rates: u32) {
    unsafe { serial_begin(baud_rates) }
}
/// Disables serial communication, allowing the RX and TX pins to be used for general input and output. To re-enable serial communication, call [begin()].
pub fn end() {
    unsafe { serial_end() }
}
/// Reads incoming serial data.
/// Use only inside of [available()]:
/// ```
/// if (Serial::available() > 0) {
///     // read the incoming byte:
///     let incomingByte: i16 = Serial::read();
///
///     // say what you got:
///     Serial::print("I received: ");
///     Serial::println(incomingByte);
/// }
/// ```
/// ### Returns
///
///The first byte of incoming serial data available (or -1 if no data is available). Data type: int.
pub fn read() -> i16 {
    unsafe { serial_read() }
}
/// Reads incoming serial data.
///
/// Use only inside of [available()]:
/// ```
/// if (Serial::available() > 0) {
///     // read the incoming byte:
///     let incomingByte: &str = Serial::read_as_utf8_str();
///
///     // say what you got:
///     Serial::print("I received: ");
///     Serial::println(incomingByte);
/// }
/// ```
/// ### Returns
///
///The first byte of incoming serial data available (or -1 if no data is available). Data type: &str.
pub fn read_as_utf8_str() -> &'static str {
    let intcoming_byte = unsafe { serial_read() };
    static mut L: [u8; 2] = [0, 0];
    unsafe {
        L[0] = intcoming_byte as u8;
    }
    unsafe { core::str::from_utf8(&L).unwrap() }
}

/// Get the number of bytes (characters) available for reading from the serial port. This is data that’s already arrived and stored in the serial receive buffer (which holds 64 bytes).
/// ### Example
/// ```
/// if (Serial::available() > 0) {
///     // read the incoming byte:
///     incomingByte = Serial::read();
///
///     // say what you got:
///     Serial::print("I received: ");
///     Serial::println(incomingByte);
/// }
/// ```
pub fn available() -> i16 {
    unsafe { serial_available() }
}
pub trait Serialprintlnable
where
    Self: Sized,
{
    type Parameters;

    fn println_2(self, params: Self::Parameters);
    fn default_parameters() -> Self::Parameters;

    fn println(self) {
        self.println_2(Self::default_parameters());
    }
}

impl Serialprintlnable for i16 {
    type Parameters = Base;

    fn println_2(self, params: Self::Parameters) {
        unsafe {
            println_int(self, params as c_int);
        }
    }

    fn default_parameters() -> Self::Parameters {
        Base::Dec
    }
}

impl Serialprintlnable for u16 {
    type Parameters = Base;

    fn println_2(self, params: Self::Parameters) {
        unsafe {
            println_unsigned_int(self, params as c_int);
        }
    }

    fn default_parameters() -> Self::Parameters {
        Base::Dec
    }
}

impl Serialprintlnable for i32 {
    type Parameters = Base;

    fn println_2(self, params: Self::Parameters) {
        unsafe {
            println_long(self, params as c_int);
        }
    }

    fn default_parameters() -> Self::Parameters {
        Base::Dec
    }
}

impl Serialprintlnable for u32 {
    type Parameters = Base;

    fn println_2(self, params: Self::Parameters) {
        unsafe {
            println_unsigned_long(self, params as c_int);
        }
    }

    fn default_parameters() -> Self::Parameters {
        Base::Dec
    }
}

impl Serialprintlnable for &[u8] {
    type Parameters = ();

    fn println_2(self, _params: Self::Parameters) {
        unsafe {
            println_chars(self as *const [u8] as *const i8);
        }
    }

    fn default_parameters() -> Self::Parameters {}
}

impl Serialprintlnable for &str {
    type Parameters = ();

    fn println_2(self, _params: Self::Parameters) {
        unsafe {
            println_chars(self.as_bytes() as *const [u8] as *const i8);
        }
    }

    fn default_parameters() -> Self::Parameters {}
}
impl<const N: usize> Serialprintlnable for crate::heapless::String<N> {
    type Parameters = ();

    fn println_2(self, _params: Self::Parameters) {
        unsafe {
            println_chars(self.as_bytes() as *const [u8] as *const i8);
        }
    }

    fn default_parameters() -> Self::Parameters {}
}

impl Serialprintlnable for Pstring {
    type Parameters = ();

    fn println_2(self, _params: Self::Parameters) {
        unsafe {
            println_chars_progmem(self.pointer);
        }
    }

    fn default_parameters() -> Self::Parameters {}
}

extern "C" {
    #[link_name = "arduino_serial_println_chars"]
    fn println_chars(cstr: *const c_char);
    #[doc(hidden)]
    #[link_name = "arduino_serial_println_chars_progmem"]
    fn println_chars_progmem(pstring: *const c_char);
    // #[link_name = "arduino_serial_println_char"]
    // fn println_char(c: c_char) -> c_size_t;
    #[doc(hidden)]
    #[link_name = "arduino_serial_println_int"]
    fn println_int(n: c_int, base: c_int) -> c_size_t;
    #[doc(hidden)]
    #[link_name = "arduino_serial_println_long"]
    fn println_long(n: c_long, base: c_int) -> c_size_t;
    #[doc(hidden)]
    #[link_name = "arduino_serial_println_unsigned_char"]
    fn println_unsigned_char(n: c_uchar, base: c_int) -> c_size_t;
    #[doc(hidden)]
    #[link_name = "arduino_serial_println_unsigned_int"]
    fn println_unsigned_int(n: c_uint, base: c_int) -> c_size_t;
    #[doc(hidden)]
    #[link_name = "arduino_serial_println_unsigned_long"]
    fn println_unsigned_long(n: c_ulong, base: c_int) -> c_size_t;
}

pub trait Serialprintable
where
    Self: Sized,
{
    type Parameters;

    fn print_2(self, params: Self::Parameters);
    fn default_parameters() -> Self::Parameters;

    fn print(self) {
        self.print_2(Self::default_parameters());
    }
}

impl Serialprintable for i16 {
    type Parameters = Base;

    fn print_2(self, params: Self::Parameters) {
        unsafe {
            print_int(self, params as c_int);
        }
    }

    fn default_parameters() -> Self::Parameters {
        Base::Dec
    }
}

impl Serialprintable for u16 {
    type Parameters = Base;

    fn print_2(self, params: Self::Parameters) {
        unsafe {
            print_unsigned_int(self, params as c_int);
        }
    }

    fn default_parameters() -> Self::Parameters {
        Base::Dec
    }
}

impl Serialprintable for i32 {
    type Parameters = Base;

    fn print_2(self, params: Self::Parameters) {
        unsafe {
            print_long(self, params as c_int);
        }
    }

    fn default_parameters() -> Self::Parameters {
        Base::Dec
    }
}

impl Serialprintable for u32 {
    type Parameters = Base;

    fn print_2(self, params: Self::Parameters) {
        unsafe {
            print_unsigned_long(self, params as c_int);
        }
    }

    fn default_parameters() -> Self::Parameters {
        Base::Dec
    }
}

impl Serialprintable for &[u8] {
    type Parameters = ();

    fn print_2(self, _params: Self::Parameters) {
        unsafe {
            print_chars(self as *const [u8] as *const i8);
        }
    }

    fn default_parameters() -> Self::Parameters {}
}

impl Serialprintable for &str {
    type Parameters = ();

    fn print_2(self, _params: Self::Parameters) {
        unsafe {
            print_chars(self.as_bytes() as *const [u8] as *const i8);
        }
    }

    fn default_parameters() -> Self::Parameters {}
}
impl<const N: usize> Serialprintable for crate::heapless::String<N> {
    type Parameters = ();

    fn print_2(self, _params: Self::Parameters) {
        unsafe {
            print_chars(self.as_bytes() as *const [u8] as *const i8);
        }
    }

    fn default_parameters() -> Self::Parameters {}
}

impl Serialprintable for Pstring {
    type Parameters = ();

    fn print_2(self, _params: Self::Parameters) {
        unsafe {
            print_chars_progmem(self.pointer);
        }
    }

    fn default_parameters() -> Self::Parameters {}
}

extern "C" {
    #[link_name = "arduino_serial_print_chars"]
    fn print_chars(cstr: *const c_char);
    #[doc(hidden)]
    #[link_name = "arduino_serial_print_chars_progmem"]
    fn print_chars_progmem(pstring: *const c_char);
    // #[link_name = "arduino_serial_print_char"]
    // fn print_char(c: c_char) -> c_size_t;
    #[doc(hidden)]
    #[link_name = "arduino_serial_print_int"]
    fn print_int(n: c_int, base: c_int) -> c_size_t;
    #[doc(hidden)]
    #[link_name = "arduino_serial_print_long"]
    fn print_long(n: c_long, base: c_int) -> c_size_t;
    #[doc(hidden)]
    #[link_name = "arduino_serial_print_unsigned_char"]
    fn print_unsigned_char(n: c_uchar, base: c_int) -> c_size_t;
    #[doc(hidden)]
    #[link_name = "arduino_serial_print_unsigned_int"]
    fn print_unsigned_int(n: c_uint, base: c_int) -> c_size_t;
    #[doc(hidden)]
    #[link_name = "arduino_serial_print_unsigned_long"]
    fn print_unsigned_long(n: c_ulong, base: c_int) -> c_size_t;
}
