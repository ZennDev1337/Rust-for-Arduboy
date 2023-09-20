//! This is the Module to interact in a save way with the Arduino Serial C++ library.
//!
//! You will need to uncomment the Arduino_Serial_Library in the import_config.h file.
use crate::prelude::Pstring;
use core::ffi::{c_char, c_int, c_long, c_size_t, c_uchar, c_uint, c_ulong};

use crate::print::Base;

extern "C" {
    #[link_name = "arduino_serial_begin"]
    fn serial_begin(serial: c_ulong);
    #[link_name = "arduino_serial_println_chars"]
    fn print_chars(cstr: *const c_char);
    #[doc(hidden)]
    #[link_name = "arduino_serial_println_chars_progmem"]
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
pub fn print(x: impl SerialPrintable) {
    x.print()
}
/// Set the Baud rate for the serial monitor
///
/// ### Example
/// ```
/// serial::begin(9600)
/// ```
pub fn begin(baud_rates: u32) {
    unsafe { serial_begin(baud_rates) }
}
pub trait SerialPrintable
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

impl SerialPrintable for i16 {
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

impl SerialPrintable for u16 {
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

impl SerialPrintable for i32 {
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

impl SerialPrintable for u32 {
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

impl SerialPrintable for &[u8] {
    type Parameters = ();

    fn print_2(self, _params: Self::Parameters) {
        unsafe {
            print_chars(self as *const [u8] as *const i8);
        }
    }

    fn default_parameters() -> Self::Parameters {}
}

impl SerialPrintable for &str {
    type Parameters = ();

    fn print_2(self, _params: Self::Parameters) {
        unsafe {
            print_chars(self.as_bytes() as *const [u8] as *const i8);
        }
    }

    fn default_parameters() -> Self::Parameters {}
}
impl<const N: usize> SerialPrintable for crate::heapless::String<N> {
    type Parameters = ();

    fn print_2(self, _params: Self::Parameters) {
        unsafe {
            print_chars(self.as_bytes() as *const [u8] as *const i8);
        }
    }

    fn default_parameters() -> Self::Parameters {}
}

impl SerialPrintable for Pstring {
    type Parameters = ();

    fn print_2(self, _params: Self::Parameters) {
        unsafe {
            print_chars_progmem(self.pointer);
        }
    }

    fn default_parameters() -> Self::Parameters {}
}
