//! Consts given by the ArduboyFX library.
//!
//! You can use the `use arduboyfx::fx_consts::*;` module to access the consts.
//! ```
//! use arduboy_rust::prelude::*;
//! use arduboyfx::fx_consts::*;
//!
//! fn setup(){
//!     let demo = dbmBlack;
//! }
//!
//! ```
#![allow(non_upper_case_globals)]
pub const dbfWhiteBlack: u8 = 0;
pub const dbfInvert: u8 = 0;
pub const dbfBlack: u8 = 0;
pub const dbfReverseBlack: u8 = 3;
pub const dbfMasked: u8 = 4;
pub const dbfFlip: u8 = 5;
pub const dbfExtraRow: u8 = 7;
pub const dbfEndFrame: u8 = 6;
pub const dbfLastFrame: u8 = 7;
pub const dbmBlack: u8 = (1 << dbfReverseBlack) | (1 << dbfBlack) | (1 << dbfWhiteBlack);
pub const dbmWhite: u8 = 1 << dbfWhiteBlack;
pub const dbmInvert: u8 = 1 << dbfInvert;
pub const dbmFlip: u8 = 1 << dbfFlip;
pub const dbmNormal: u8 = 0;
pub const dbmOverwrite: u8 = 0;
pub const dbmReverse: u8 = 1 << dbfReverseBlack;
pub const dbmMasked: u8 = 1 << dbfMasked;
pub const dbmEndFrame: u8 = 1 << dbfEndFrame;
pub const dbmLastFrame: u8 = 1 << dbfLastFrame;
pub const dcfWhiteBlack: u8 = 0;
pub const dcfInvert: u8 = 1;
pub const dcfBlack: u8 = 2;
pub const dcfReverseBlack: u8 = 3;
pub const dcfMasked: u8 = 4;
pub const dcfProportional: u8 = 5;
pub const dcmBlack: u8 = (1 << dcfReverseBlack) | (1 << dcfBlack) | (1 << dcfWhiteBlack);
pub const dcmWhite: u8 = 1 << dcfWhiteBlack;
pub const dcmInvert: u8 = 1 << dcfInvert;
pub const dcmNormal: u8 = 0;
pub const dcmOverwrite: u8 = 0;
pub const dcmReverse: u8 = 1 << dcfReverseBlack;
pub const dcmMasked: u8 = 1 << dcfMasked;
pub const dcmProportional: u8 = 1 << dcfProportional;
pub const FX_VECTOR_KEY_VALUE: u16 = 0x9518;
pub const FX_DATA_VECTOR_KEY_POINTER: u16 = 0x0014;
pub const FX_DATA_VECTOR_PAGE_POINTER: u16 = 0x0016;
pub const FX_SAVE_VECTOR_KEY_POINTER: u16 = 0x0018;
pub const FX_SAVE_VECTOR_PAGE_POINTER: u16 = 0x001A;
pub const SFC_JEDEC_ID: u8 = 0x9F;
pub const SFC_READSTATUS1: u8 = 0x05;
pub const SFC_READSTATUS2: u8 = 0x35;
pub const SFC_READSTATUS3: u8 = 0x15;
pub const SFC_READ: u8 = 0x03;
pub const SFC_WRITE_ENABLE: u8 = 0x06;
pub const SFC_WRITE: u8 = 0x02;
pub const SFC_ERASE: u8 = 0x20;
pub const SFC_RELEASE_POWERDOWN: u8 = 0xAB;
pub const SFC_POWERDOWN: u8 = 0xB9;
