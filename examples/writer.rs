// ferris.png source: https://www.kickstarter.com/projects/1702984242/ferris-the-small-squishable-rustacean-rust-mascot

use hanbun::{self, Color};
use std::{
    env,
    io::{BufWriter, Write},
    process,
};

fn main() {
    let mut args = env::args();
    args.next().expect("Executable path non-existent");

    if let (Some(width), Some(height), Some(image_path)) = (args.next(), args.next(), args.next()) {
        let size: (usize, usize) = (width.parse().unwrap(), height.parse().unwrap());
        let writer = BufWriter::with_capacity(size.0 * size.1, Vec::new());
        let writer = convert(size, image_path, writer);
        let bytes = writer.into_inner().unwrap();

        eprintln!("Image converted to halfblocks ({} bytes).", bytes.len());
        unsafe {
            let str = String::from_utf8_unchecked(bytes.to_vec());
            println!("{str}");
        }
    } else {
        eprintln!("Width, Height, and Image path missing")
    }
    process::exit(1);
}

fn convert<W: Write>(
    size: (usize, usize),
    image_path: String,
    writer: BufWriter<W>,
) -> BufWriter<W> {
    let mut buffer = hanbun::Buffer::with_writer(size.0, size.1, ' ', writer);
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
        } else {
            eprintln!("Image decoding failed")
        }
    } else {
        eprintln!("Image loading failed")
    }
    buffer.writer
}
