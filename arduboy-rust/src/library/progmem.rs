#![allow(unused_imports)]
/// Create a space for Progrem variable
/// ## Example
/// ```
/// //for text
/// progmem!(
///     static text: [u8; _] = *b"I'm a PROGMEM Text\0";
/// );
/// //for tone sequence
/// progmem!(
///     static tone: [u16; 43] = [
///         NOTE_E4, 400, NOTE_D4, 200, NOTE_C4, 400, NOTE_D4, 200, NOTE_C4, 300, NOTE_REST,
///     ];
/// );
/// //for for bitmap
/// progmem!(
///     static image: [u8; _] = [8, 8, 0x81, 0x00, 0x12, 0x40, 0x04, 0x11, 0x00, 0x04];
/// );
/// ```
#[macro_export]
macro_rules! progmem {
    (
        $( #[$attr:meta] )*
        $v:vis $id:ident $name:ident: [$ty:ty; _] = $value:expr;
        $($rest:tt)*
    ) => {
        $( #[$attr] )*
        #[link_section = ".progmem.data"]
        $v $id $name: [$ty; $value.len()] = $value;
        $crate::progmem!{
			$($rest)*
		}
    };
    () => ()
}

pub(super) use progmem;
///Create a `const` raw pointer to a sprite as u8, without creating an intermediate reference.
#[macro_export]
macro_rules! get_sprite_addr {
    ( $s:expr ) => {
        addr_of!($s) as *const u8
    };
}
pub(super) use get_sprite_addr;

///Create a `const` raw pointer to a sprite as u16, without creating an intermediate reference.
#[macro_export]
macro_rules! get_tones_addr {
    ( $s:expr ) => {
        addr_of!($s) as *const u16
    };
}
pub(super) use get_tones_addr;

///Create a `const` raw pointer to a \[u8;_] that saves text, without creating an intermediate reference.
#[macro_export]
macro_rules! get_string_addr {
    ( $s:expr ) => {
        Pstring {
            pointer: addr_of!($s) as *const i8,
        }
    };
}
pub(super) use get_string_addr;
