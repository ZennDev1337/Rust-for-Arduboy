use crate::libraries::arduino_system::progmem::Pstring;
use core::ffi::{c_char, c_uchar, c_ulong};

pub trait DrawableString
where
    Self: Sized,
{
    fn draw(self);
}
impl DrawableString for &[u8] {
    fn draw(self) {
        unsafe {
            arduboyfx_draw_string(self as *const [u8] as *const i8);
        }
    }
}
impl DrawableString for &str {
    fn draw(self) {
        unsafe {
            arduboyfx_draw_string(self.as_bytes() as *const [u8] as *const i8);
        }
    }
}
impl<const N: usize> DrawableString for crate::heapless::String<N> {
    fn draw(self) {
        unsafe {
            arduboyfx_draw_string(self.as_bytes() as *const [u8] as *const i8);
        }
    }
}
impl DrawableString for Pstring {
    fn draw(self) {
        unsafe {
            arduboyfx_draw_string_buffer(self.pointer as *const u8);
        }
    }
}
impl DrawableString for u32 {
    fn draw(self) {
        unsafe {
            arduboyfx_draw_string_fx(self);
        }
    }
}

extern "C" {
    #[link_name = "arduboyfx_draw_string_fx"]
    fn arduboyfx_draw_string_fx(address: c_ulong);
    #[link_name = "arduboyfx_draw_string_buffer"]
    fn arduboyfx_draw_string_buffer(buffer: *const c_uchar);
    #[link_name = "arduboyfx_draw_string"]
    fn arduboyfx_draw_string(cstr: *const c_char);
}
