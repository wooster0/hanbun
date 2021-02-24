// ferris.png source: https://www.kickstarter.com/projects/1702984242/ferris-the-small-squishable-rustacean-rust-mascot

use hanbun::{self, Color};
use std::{env, process};

fn main() {
    let size = hanbun::size().unwrap_or_else(|_| {
        eprintln!("Unknown terminal size");
        process::exit(1)
    });

    let mut args = env::args();
    args.next().expect("Executable path non-existent");

    let mut buffer = hanbun::Buffer::new(size.0, size.1, ' ');

    if let Some(image_path) = args.next() {
        if let Ok(image) = image::io::Reader::open(image_path) {
            if let Ok(decoded_image) = image.decode() {
                let resized_image = decoded_image.resize(
                    (size.0 as f64 * 1.5) as u32,
                    (size.1 as f64 * 1.5) as u32,
                    image::imageops::FilterType::Triangle,
                );
                for (y, row) in resized_image.to_rgb8().rows().enumerate() {
                    for (x, pixel) in row.enumerate() {
                        buffer.color(
                            x,
                            y,
                            Color::Rgb {
                                r: pixel[0],
                                g: pixel[1],
                                b: pixel[2],
                            },
                        );
                    }
                }
                buffer.draw();
                buffer.clear(' ');

                return;
            } else {
                eprintln!("Image decoding failed")
            }
        } else {
            eprintln!("Image loading failed")
        }
    } else {
        eprintln!("Image path missing")
    }

    process::exit(1);
}
