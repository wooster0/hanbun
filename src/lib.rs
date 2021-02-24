// Some trivia:
// terminal = text input/output environment
// console = physical terminal

//! Welcome to the top of the hanbun crate.
//! [`Buffer`] should be of interest to you.

use crossterm::queue;
use crossterm::style::{ResetColor, SetBackgroundColor, SetForegroundColor};
use std::io::{self, stdout, BufWriter, Write};

/// Returns the terminal's width and height.
///
/// # Errors
///
/// May return an error if the operation failed.
pub fn size() -> Result<(usize, usize), ()> {
    if let Ok((width, height)) = crossterm::terminal::size() {
        Ok((width as usize, height as usize))
    } else {
        Err(())
    }
}

/// Represents a terminal cell.
#[derive(Debug, Clone)]
pub struct Cell {
    pub upper_block: Option<Option<crossterm::style::Color>>,
    pub lower_block: Option<Option<crossterm::style::Color>>,
    /// The fallback character displayed if both [`Cell::upper_block`] and [`Cell::lower_block`] are [`None`].
    pub char: char,
}

/// A buffer for storing the state of the cells.
/// You can see it as a drawing canvas.
///
/// # Examples
///
/// ```
/// let mut buffer;
///
/// if let Ok((width, height)) = hanbun::size() {
///     buffer = hanbun::Buffer::new(width, height, ' ');
/// } else {
///     return;
/// }
///
/// buffer.set(3, 3);
/// buffer.draw();
/// ```
pub struct Buffer {
    pub cells: Vec<Cell>,
    writer: BufWriter<io::Stdout>,
    pub width: usize,
    pub height: usize,
}

/// See [this list](https://docs.rs/crossterm/0.19.0/crossterm/style/enum.Color.html) for all available colors.
pub type Color = crossterm::style::Color;

impl Buffer {
    /// Creates a new buffer of `width * height` cells filled with `char`.
    pub fn new(width: usize, height: usize, char: char) -> Buffer {
        Buffer {
            cells: vec![
                Cell {
                    upper_block: None,
                    lower_block: None,
                    char
                };
                width * height
            ],
            writer: BufWriter::with_capacity(width * height, stdout()),
            width,
            height,
        }
    }

    /// Draws the buffer to the screen.
    ///
    /// # Panics
    ///
    /// Panics if an internal write operation operation failed.
    pub fn draw(&mut self) {
        let writer = &mut self.writer;
        let mut x = 0;
        let mut y = 1;
        for cell in &self.cells {
            if cell.upper_block.is_some() && cell.lower_block.is_some() {
                if let Some(Some(upper_color)) = cell.upper_block {
                    if let Some(Some(lower_color)) = cell.lower_block {
                        queue!(writer, SetForegroundColor(upper_color)).unwrap();
                        queue!(writer, SetBackgroundColor(lower_color)).unwrap();
                        writer.write_all("▀".as_bytes()).unwrap();
                    } else {
                        queue!(writer, SetBackgroundColor(upper_color)).unwrap();
                        writer.write_all("▄".as_bytes()).unwrap();
                    }
                    queue!(writer, ResetColor).unwrap();
                } else if let Some(Some(lower_color)) = cell.lower_block {
                    if let Some(Some(upper_color)) = cell.upper_block {
                        queue!(writer, SetForegroundColor(upper_color)).unwrap();
                        queue!(writer, SetBackgroundColor(lower_color)).unwrap();
                        writer.write_all("▀".as_bytes()).unwrap();
                    } else {
                        queue!(writer, SetBackgroundColor(lower_color)).unwrap();
                        writer.write_all("▀".as_bytes()).unwrap();
                    }
                    queue!(writer, ResetColor).unwrap();
                } else {
                    writer.write_all("█".as_bytes()).unwrap();
                }
            } else if let Some(upper_block) = cell.upper_block {
                if let Some(color) = upper_block {
                    queue!(writer, SetForegroundColor(color)).unwrap();
                }
                writer.write_all("▀".as_bytes()).unwrap();
                if upper_block.is_some() {
                    queue!(writer, ResetColor).unwrap();
                }
            } else if let Some(lower_block) = cell.lower_block {
                if let Some(color) = lower_block {
                    queue!(writer, SetForegroundColor(color)).unwrap();
                }
                writer.write_all("▄".as_bytes()).unwrap();
                if lower_block.is_some() {
                    queue!(writer, ResetColor).unwrap();
                }
            } else {
                write!(writer, "{}", cell.char).unwrap();
            }

            x += 1;
            if y != self.height && x == self.width {
                writer.write_all(b"\n").unwrap();
                x = 0;
                y += 1;
            }
        }
        self.writer.flush().unwrap();
    }

    /// Clears the buffer using `char`.
    pub fn clear(&mut self, char: char) {
        self.cells.fill(Cell {
            upper_block: None,
            lower_block: None,
            char,
        })
    }

    /// Sets the cell at (`x`, `y`) to a half block.
    ///
    /// # Panics
    ///
    /// Panics if (`x`, `y`) is out of the buffer's range.
    pub fn set(&mut self, x: usize, y: usize) {
        let position = x + self.width * (y / 2);
        let current_cell = &self
            .cells
            .get(position)
            .expect(&format!("setting block at ({}, {}) (out of range)", x, y));

        if y % 2 == 0 {
            self.cells[position] = Cell {
                upper_block: Some(None),
                lower_block: current_cell.lower_block,
                char: ' ',
            };
        } else {
            self.cells[position] = Cell {
                upper_block: current_cell.upper_block,
                lower_block: Some(None),
                char: ' ',
            };
        }
    }

    /// Colors the cell at (`x`, `y`) with the given color.
    ///
    /// # Panics
    ///
    /// Panics if (`x`, `y`) is out of the buffer's range.
    pub fn color(&mut self, x: usize, y: usize, color: Color) {
        let position = x + self.width * (y / 2);
        let current_cell = &self
            .cells
            .get(position)
            .expect(&format!("coloring block at ({}, {}) (out of range)", x, y));

        if y % 2 == 0 {
            self.cells[position] = Cell {
                upper_block: Some(Some(color)),
                lower_block: current_cell.lower_block,
                char: ' ',
            };
        } else {
            self.cells[position] = Cell {
                upper_block: current_cell.upper_block,
                lower_block: Some(Some(color)),
                char: ' ',
            };
        }
    }

    /// Prints string to (`x`, `y`).
    ///
    /// # Panics
    ///
    /// Panics if (`x`, `y`) is out of the buffer's range.
    pub fn print(&mut self, x: usize, y: usize, string: &str) {
        let position = x + self.width * (y / 2);

        for (index, char) in string.chars().enumerate() {
            let cell = self
                .cells
                .get_mut(index + position)
                .expect(&format!("printing at ({}, {}) (out of range)", x, y));

            *cell = Cell {
                upper_block: None,
                lower_block: None,
                char,
            };
        }
    }
}
