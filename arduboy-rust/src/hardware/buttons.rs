//! A list of all six buttons available on the Arduboy
/// Just a `const` for the UP button
pub const UP: ButtonSet = ButtonSet {
    flag_set: 0b10000000,
};
/// Just a `const` for the RIGHT button
pub const RIGHT: ButtonSet = ButtonSet {
    flag_set: 0b01000000,
};
/// Just a `const` for the LEFT button
pub const LEFT: ButtonSet = ButtonSet {
    flag_set: 0b00100000,
};
/// Just a `const` for the DOWN button
pub const DOWN: ButtonSet = ButtonSet {
    flag_set: 0b00010000,
};
/// Just a `const` for the A button
pub const A: ButtonSet = ButtonSet {
    flag_set: 0b00001000,
};
/// Just a `const` for the B button
pub const B: ButtonSet = ButtonSet {
    flag_set: 0b00000100,
};
/// Just a `const` for the any
pub const ANY_BUTTON: ButtonSet = ButtonSet {
    flag_set: 0b11111111,
};
/// Just a `const` for the UP button
pub const UP_BUTTON: ButtonSet = UP;
/// Just a `const` for the RIGHT button
pub const RIGHT_BUTTON: ButtonSet = RIGHT;
/// Just a `const` for the DOWN button
pub const DOWN_BUTTON: ButtonSet = DOWN;
/// Just a `const` for the LEFT button
pub const LEFT_BUTTON: ButtonSet = LEFT;
/// Just a `const` for the A button
pub const A_BUTTON: ButtonSet = A;
/// Just a `const` for the B button
pub const B_BUTTON: ButtonSet = B;
///This struct gives the library a understanding what Buttons on the Arduboy are.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct ButtonSet {
    pub flag_set: u8,
}

impl ButtonSet {
    pub unsafe fn pressed(&self) -> bool {
        crate::library::arduboy2::pressed(self.flag_set)
    }

    pub unsafe fn just_pressed(&self) -> bool {
        crate::library::arduboy2::just_pressed(self.flag_set)
    }

    pub unsafe fn just_released(&self) -> bool {
        crate::library::arduboy2::just_released(self.flag_set)
    }
    pub unsafe fn not_pressed(&self) -> bool {
        crate::library::arduboy2::not_pressed(self.flag_set)
    }
}

impl core::ops::BitOr for ButtonSet {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        Self {
            flag_set: self.flag_set | other.flag_set,
        }
    }
}
