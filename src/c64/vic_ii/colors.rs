use std::str::FromStr;

enum Colors {
    Black = 0x0,
    White = 0x1,
    Red = 0x2,
    Cyan = 0x3,
    Purple = 0x4,
    Green = 0x5,
    Blue = 0x6,
    Yellow = 0x7,
    Orange = 0x8,
    Brown = 0x9,
    LightRed = 0xa,
    DarkGrey = 0xb,
    Grey = 0xc,
    LightGreen = 0xd,
    LightBlue = 0xe,
    LightGrey = 0xf,
}

struct Palette {
    black: Color,
    white: Color,
    red: Color,
    cyan: Color,
    purple: Color,
    green: Color,
    blue: Color,
    yellow: Color,
    orange: Color,
    brown: Color,
    light_red: Color,
    dark_grey: Color,
    grey: Color,
    light_green: Color,
    light_blue: Color,
    light_grey: Color,
}

#[rustfmt::skip]
static RAW_PALETTE: Palette = Palette {
    black:        Color::hex(0x000000),
    white:        Color::hex(0xffffff),
    red:          Color::hex(0x68372b),
    cyan:         Color::hex(0x70a4b2),
    purple:       Color::hex(0x6f3d86),
    green:        Color::hex(0x588d43),
    blue:         Color::hex(0x352879),
    yellow:       Color::hex(0xb8c76f),
    orange:       Color::hex(0x6f4f25),
    brown:        Color::hex(0x433900),
    light_red:    Color::hex(0x9a6759),
    dark_grey:    Color::hex(0x444444),
    grey:         Color::hex(0x6c6c6c),
    light_green:  Color::hex(0x9ad284),
    light_blue:   Color::hex(0x6c5eb5),
    light_grey:   Color::hex(0x959595),
};

/*
  This palette takes it's color values from this article:
  https://www.pepto.de/projects/colorvic/2001/
*/
#[rustfmt::skip]
static GAMMA_CORRECTED_PALETTE: Palette = Palette {
    black:        Color::hex(0x000000),
    white:        Color::hex(0xffffff),
    red:          Color::hex(0x68372b),
    cyan:         Color::hex(0x70a4b2),
    purple:       Color::hex(0x6f3d86),
    green:        Color::hex(0x588d43),
    blue:         Color::hex(0x352879),
    yellow:       Color::hex(0xb8c76f),
    orange:       Color::hex(0x6f4f25),
    brown:        Color::hex(0x433900),
    light_red:    Color::hex(0x9a6759),
    dark_grey:    Color::hex(0x444444),
    grey:         Color::hex(0x6c6c6c),
    light_green:  Color::hex(0x9ad284),
    light_blue:   Color::hex(0x6c5eb5),
    light_grey:   Color::hex(0x959595),
};

#[derive(Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
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
