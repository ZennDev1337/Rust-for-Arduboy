#![allow(non_upper_case_globals)]
use core::ffi::{c_int, c_long, c_uchar, c_uint, c_ulong};

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
    #[link_name = "arduboyfx_draw_bitmap"]
    pub fn arduboyfx_draw_bitmap(
        x: c_int,
        y: c_int,
        address: c_ulong,
        frame: c_uchar,
        mode: c_uchar,
    );
}
