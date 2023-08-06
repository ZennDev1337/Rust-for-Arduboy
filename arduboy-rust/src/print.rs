use core::ffi::c_int;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Base {
    Bin = 2,
    Oct = 8,
    Dec = 10,
    Hex = 16,
}

pub trait Printable
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

impl Printable for i16 {
    type Parameters = Base;

    fn print_2(self, params: Self::Parameters) {
        unsafe {
            crate::library::arduboy::print_int(self, params as c_int);
        }
    }

    fn default_parameters() -> Self::Parameters {
        Base::Dec
    }
}

impl Printable for u16 {
    type Parameters = Base;

    fn print_2(self, params: Self::Parameters) {
        unsafe {
            crate::library::arduboy::print_unsigned_int(self, params as c_int);
        }
    }

    fn default_parameters() -> Self::Parameters {
        Base::Dec
    }
}

impl Printable for i32 {
    type Parameters = Base;

    fn print_2(self, params: Self::Parameters) {
        unsafe {
            crate::library::arduboy::print_long(self, params as c_int);
        }
    }

    fn default_parameters() -> Self::Parameters {
        Base::Dec
    }
}

impl Printable for u32 {
    type Parameters = Base;

    fn print_2(self, params: Self::Parameters) {
        unsafe {
            crate::library::arduboy::print_unsigned_long(self, params as c_int);
        }
    }

    fn default_parameters() -> Self::Parameters {
        Base::Dec
    }
}

impl Printable for &[u8] {
    type Parameters = ();

    fn print_2(self, _params: Self::Parameters) {
        unsafe {
            crate::library::arduboy::print_chars(self as *const [u8] as *const i8);
        }
    }

    fn default_parameters() -> Self::Parameters {}
}

impl Printable for &str {
    type Parameters = ();

    fn print_2(self, _params: Self::Parameters) {
        unsafe {
            crate::library::arduboy::print_chars(self.as_bytes() as *const [u8] as *const i8);
        }
    }

    fn default_parameters() -> Self::Parameters {}
}
