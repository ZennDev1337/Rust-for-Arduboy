use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong};
pub trait DrawableNumber
where
    Self: Sized,
{
    fn draw(self, digits: i8);
}
impl DrawableNumber for i16 {
    fn draw(self, digits: i8) {
        unsafe { arduboyfx_draw_number_i16(self, digits) }
    }
}
impl DrawableNumber for u16 {
    fn draw(self, digits: i8) {
        unsafe { arduboyfx_draw_number_u16(self, digits) }
    }
}
impl DrawableNumber for i32 {
    fn draw(self, digits: i8) {
        unsafe { arduboyfx_draw_number_i32(self, digits) }
    }
}
impl DrawableNumber for u32 {
    fn draw(self, digits: i8) {
        unsafe { arduboyfx_draw_number_u32(self, digits) }
    }
}

extern "C" {
    #[link_name = "arduboyfx_draw_number_i16"]
    fn arduboyfx_draw_number_i16(n: c_int, digits: c_char);
    #[link_name = "arduboyfx_draw_number_u16"]
    fn arduboyfx_draw_number_u16(n: c_uint, digits: c_char);
    #[link_name = "arduboyfx_draw_number_i32"]
    fn arduboyfx_draw_number_i32(n: c_long, digits: c_char);
    #[link_name = "arduboyfx_draw_number_u32"]
    fn arduboyfx_draw_number_u32(n: c_ulong, digits: c_char);
}
