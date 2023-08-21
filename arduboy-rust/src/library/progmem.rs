#![allow(unused_imports)]
/// Create a space for Progmem variable
/// ## Example
/// ```
/// //for text
/// progmem!(
///     static text: [u8; _] = *b"I'm a PROGMEM Text\0";
/// );
/// //for tone sequence
/// progmem!(
///     static tone: [u16; _] = [
///         NOTE_E4, 400, NOTE_D4, 200, NOTE_C4, 400, NOTE_D4, 200, NOTE_C4, 300, NOTE_REST,
///     ];
/// );
/// //for for bitmap
/// progmem!(
///     static image: [u8; _] = [8, 8, 0x81, 0x00, 0x12, 0x40, 0x04, 0x11, 0x00, 0x04];
/// );
///
/// // for a Vector
/// progmem!(
///     static mut walls: Vec<Player, 100> = Vec::new();
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
    (
        $( #[$attr:meta] )*
        $v:vis $id:ident mut $name:ident: [$ty:ty; _] = $value:expr;
        $($rest:tt)*
    ) => {
        $( #[$attr] )*
        #[link_section = ".progmem.data"]
        $v $id mut $name: [$ty; $value.len()] = $value;
        $crate::progmem!{
			$($rest)*
		}
    };
    (
        $( #[$attr:meta] )*
        $v:vis $id:ident $name:ident: $ty:ty = $value:expr;
        $($rest:tt)*
    ) => {
        $( #[$attr] )*
        #[link_section = ".progmem.data"]
        $v $id $name: $ty = $value;
        $crate::progmem!{
			$($rest)*
		}
    };
    (
        $( #[$attr:meta] )*
        $v:vis $id:ident mut $name:ident: $ty:ty = $value:expr;
        $($rest:tt)*
    ) => {
        $( #[$attr] )*
        #[link_section = ".progmem.data"]
        $v $id mut $name: $ty = $value;
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
        unsafe { addr_of!($s) as *const u8 }
    };
}
pub(super) use get_sprite_addr;

///Create a `const` raw pointer to a sprite as u16, without creating an intermediate reference.
#[macro_export]
macro_rules! get_tones_addr {
    ( $s:expr ) => {
        unsafe { addr_of!($s) as *const u16 }
    };
}
pub(super) use get_tones_addr;

///Create a `const` raw pointer to a \[u8;_] that saves text, without creating an intermediate reference.
#[macro_export]
macro_rules! get_string_addr {
    ( $s:expr ) => {
        Pstring {
            pointer: unsafe { addr_of!($s) as *const i8 },
        }
    };
}
pub(super) use get_string_addr;
///This is the way to go if you want print some random text
///
/// This doesn't waste the 2kb ram it saves to progmem (28kb)
/// This automatically saves the given text to the Progmem.
/// ## Example
/// ```
/// arduboy.print(f!(b"Random text to print\0"))
/// ```
#[macro_export]
macro_rules! f {
    ($string_literal:literal) => {{
        progmem!(
            static local: [u8; _] = *$string_literal;
        );

        get_string_addr!(local)
    }};
}
pub(super) use f;

/// This struct is important for the Progmem functionality.
///
/// Typically you will never use this by your self.
/// It will be used by the get_string_addr macro in combination with a print command.
#[derive(Copy, Clone)]
pub struct Pstring {
    pub pointer: *const i8,
}
