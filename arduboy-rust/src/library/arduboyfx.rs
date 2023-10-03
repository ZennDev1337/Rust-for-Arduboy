#![allow(non_upper_case_globals)]
use core::ffi::{c_char, c_int, c_size_t, c_uchar, c_uint, c_ulong};

pub const dbmNormal: u8 = 0;
pub const dbmOverwrite: u8 = 0;
pub const dbmMasked: u8 = 1;
extern "C" {
    #[link_name = "arduboyfx_begin"]
    pub fn arduboyfx_begin();
    #[link_name = "arduboyfx_begin_data"]
    pub fn arduboyfx_begin_data(datapage: c_uint);
    #[link_name = "arduboyfx_begin_data_save"]
    pub fn arduboyfx_begin_data_save(datapage: c_uint, savepage: c_uint);
    #[link_name = "arduboyfx_display"]
    pub fn arduboyfx_display();
    #[link_name = "arduboyfx_display_clear"]
    pub fn arduboyfx_display_clear();
    #[link_name = "arduboyfx_read_data_array"]
    pub fn arduboyfx_read_data_array(
        address: c_ulong,
        index: c_uchar,
        offset: c_uchar,
        element_size: c_uchar,
        buffer: *const c_uchar,
        length: c_size_t,
    );
    #[link_name = "arduboyfx_draw_bitmap"]
    pub fn arduboyfx_draw_bitmap(
        x: c_int,
        y: c_int,
        address: c_ulong,
        frame: c_uchar,
        mode: c_uchar,
    );
    #[link_name = "arduboyfx_set_frame"]
    pub fn arduboyfx_set_frame(frame: c_ulong, repeat: c_uchar);
    #[link_name = "arduboyfx_draw_frame"]
    pub fn arduboyfx_draw_frame(address: c_ulong) -> c_ulong;
    #[link_name = "arduboyfx_draw_string"]
    pub fn arduboyfx_draw_string(address: c_ulong);
    #[link_name = "arduboyfx_draw_string_buffer"]
    pub fn arduboyfx_draw_string_buffer(buffer: *const c_uchar);
    #[link_name = "arduboyfx_set_cursor"]
    pub fn arduboyfx_set_cursor(x: c_int, y: c_int);
}
