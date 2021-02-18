use hanbun::{self, Color};

use font8x8::{UnicodeFonts, HIRAGANA_FONTS};

fn main() {
    let size = hanbun::size();
    let width = if let Ok((width, _)) = size { width } else { 50 };
    let height = 5;
    let mut cells = hanbun::Buffer::new(width as usize, height, ' ');

    let mut length = 0;
    let mut alignment = 0;
    for char in "ひらがな".chars() {
        let glyph = if let Some(glyph) = HIRAGANA_FONTS.get(char) {
            glyph
        } else {
            eprintln!("Unable to retrieve {}", char);
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
