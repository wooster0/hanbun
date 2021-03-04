use hanbun::{self, Color};

use font8x8::{UnicodeFonts, HIRAGANA_FONTS};

use std::env;

fn main() {
    let size = hanbun::size();
    let width = if let Ok((width, _height)) = size {
        width
    } else {
        50
    };
    let height = 5;
    let mut cells = hanbun::Buffer::new(width as usize, height, ' ');

    let mut args = env::args();
    args.next().expect("Executable path non-existent");
    let hiragana = args.next().unwrap_or(String::from("ひらがな"));

    let mut length = 0;
    let mut alignment = 0;
    for char in hiragana.chars() {
        let glyph = if let Some(glyph) = HIRAGANA_FONTS.get(char) {
            glyph
        } else {
            eprintln!("Unknown hiragana: {}", char);
            std::process::exit(1);
        };

        for (y, row) in glyph.iter().enumerate() {
            for bit in 0..8 {
                match *row & 1 << bit as usize {
                    0 => {}
                    _ => {
                        let x = alignment + bit;
                        if x > length {
                            length = x;
                        }
                        cells.color(x, y, Color::Green);
                    }
                }
            }
        }

        alignment += 9;
    }

    for x in 0..=length {
        cells.color(x, 9, Color::DarkGreen);
    }

    cells.draw();
}
