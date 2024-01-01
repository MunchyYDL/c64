use std::str::FromStr;

use crate::C64::Byte;

enum Colors {
    black = 0x0,
    white = 0x1,
    red = 0x2,
    cyan = 0x3,
    purple = 0x4,
    green = 0x5,
    blue = 0x6,
    yellow = 0x7,
    orange = 0x8,
    brown = 0x9,
    light_red = 0xa,
    dark_grey = 0xb,
    grey = 0xc,
    light_green = 0xd,
    light_blue = 0xe,
    light_grey = 0xf,
}

struct Palette {
    Black: Color,
    White: Color,
    Red: Color,
    Cyan: Color,
    Purple: Color,
    Green: Color,
    Blue: Color,
    Yellow: Color,
    Orange: Color,
    Brown: Color,
    Light_Red: Color,
    Dark_Grey: Color,
    Grey: Color,
    Light_Green: Color,
    Light_Blue: Color,
    Light_Grey: Color,
}

#[rustfmt::skip]
static RAW_PALETTE: Palette = Palette {
    Black:        Color::hex(0x000000),
    White:        Color::hex(0xffffff),
    Red:          Color::hex(0x68372b),
    Cyan:         Color::hex(0x70a4b2),
    Purple:       Color::hex(0x6f3d86),
    Green:        Color::hex(0x588d43),
    Blue:         Color::hex(0x352879),
    Yellow:       Color::hex(0xb8c76f),
    Orange:       Color::hex(0x6f4f25),
    Brown:        Color::hex(0x433900),
    Light_Red:    Color::hex(0x9a6759),
    Dark_Grey:    Color::hex(0x444444),
    Grey:         Color::hex(0x6c6c6c),
    Light_Green:  Color::hex(0x9ad284),
    Light_Blue:   Color::hex(0x6c5eb5),
    Light_Grey:   Color::hex(0x959595),
};

/*
  This palette takes it's color values from this article:
  https://www.pepto.de/projects/colorvic/2001/
*/
#[rustfmt::skip]
static GAMMA_CORRECTED_PALETTE: Palette = Palette {
    Black:        Color::hex(0x000000),
    White:        Color::hex(0xffffff),
    Red:          Color::hex(0x68372b),
    Cyan:         Color::hex(0x70a4b2),
    Purple:       Color::hex(0x6f3d86),
    Green:        Color::hex(0x588d43),
    Blue:         Color::hex(0x352879),
    Yellow:       Color::hex(0xb8c76f),
    Orange:       Color::hex(0x6f4f25),
    Brown:        Color::hex(0x433900),
    Light_Red:    Color::hex(0x9a6759),
    Dark_Grey:    Color::hex(0x444444),
    Grey:         Color::hex(0x6c6c6c),
    Light_Green:  Color::hex(0x9ad284),
    Light_Blue:   Color::hex(0x6c5eb5),
    Light_Grey:   Color::hex(0x959595),
};

#[derive(Debug)]
struct Color {
    r: Byte,
    g: Byte,
    b: Byte,
}

impl Color {
    #[allow(clippy::identity_op)]
    const fn hex(value: u32) -> Self {
        let r = (value >> 0x10 & 0xff) as u8;
        let g = (value >> 0x08 & 0xff) as u8;
        let b = (value >> 0x00 & 0xff) as u8;
        Color { r, g, b }
    }
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('#') || s.starts_with("0x") {
            let s = s.trim_start_matches('#').trim_start_matches("0x");
            if let Ok(val) = u32::from_str_radix(s, 16) {
                Ok(Color::hex(val))
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

#[allow(clippy::identity_op)]
impl From<u32> for Color {
    fn from(value: u32) -> Self {
        let r = (value >> 0x10 & 0xff) as u8;
        let g = (value >> 0x08 & 0xff) as u8;
        let b = (value >> 0x00 & 0xff) as u8;
        Color { r, g, b }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_construct_color_from_u32() {
        let c = Color::hex(0x345678);
        assert_eq!(c.r, 0x34);
        assert_eq!(c.g, 0x56);
        assert_eq!(c.b, 0x78);
    }

    #[test]
    fn can_construct_color_from_string() {
        let c = Color::from_str("#345678").unwrap();
        assert_eq!(c.r, 0x34);
        assert_eq!(c.g, 0x56);
        assert_eq!(c.b, 0x78);
    }

    #[test]
    fn should_return_err_if_decimal() {
        let c = Color::from_str("345678");
        assert!(c.is_err())
    }
}
