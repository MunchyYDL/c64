pub mod colors;

struct Vic {
    screen_width: u16,
    screen_height: u16,

    visible_width: u16,
    visible_height: u16,

    inner_width: u16,
    inner_height: u16,

    current_x: u16,
    current_y: u16,
}

impl Default for Vic {
    fn default() -> Self {
        Self {
            screen_width: 504,
            screen_height: 312,
            visible_width: 403,
            visible_height: 284,
            inner_width: 320,
            inner_height: 200,
            current_x: 0,
            current_y: 0,
        }
    }
}

impl Vic {
    pub fn cycle() {
        // Progress the chip by one cycle
    }
}
