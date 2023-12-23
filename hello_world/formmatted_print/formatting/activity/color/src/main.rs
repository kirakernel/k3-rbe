use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // RGB (128, 255, 90) 0x80FF5A
        write!(
            f,
            "RGB ({0}, {1}, {2}) 0x{0:02X}{1:02X}{2:02X}",
            self.red, self.green, self.blue
        )
    }
}

fn main() {
    println!("Color (RED, GREEN, BLUE) 0xFFFFFF:\n");
    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ] {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        // RGB (128, 255, 90) 0x80FF5A
        println!("{}", color);
    }
}
