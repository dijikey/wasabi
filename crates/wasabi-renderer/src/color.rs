#[macro_export]
macro_rules! color {
    ($c:ident) => {
        $c.r, $c.g, $c.b, $c.a
    };
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const RED: Color = Color::from_f32(1.0, 0.0, 0.0, 1.0);
    pub const BLUE: Color = Color::from_f32(0.0, 1.0, 0.0, 1.0);
    pub const GREEN: Color = Color::from_f32(0.0, 0.0, 1.0, 1.0);
    pub const WHITE: Color = Color::from_f32(1.0, 1.0, 1.0, 1.0);
    pub const BLACK: Color = Color::from_f32(0.0, 0.0, 0.0, 1.0);

    #[inline]
    pub const fn from_f32(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    #[inline]
    pub fn from_u8() {}
}
