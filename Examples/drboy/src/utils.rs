pub struct Position {
    x: u8,
    y: u8,
}

pub enum Color {
    Black(Position),
    White(Position),
    Gray(Position),
}
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}
pub struct Pills {
    pub first_field: Color,
    pub second_field: Color,
    pub direction: Direction,
}
