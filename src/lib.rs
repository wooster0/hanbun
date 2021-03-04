// Some trivia:
// terminal = text input/output environment
// console = physical terminal

//! Welcome to the top of the hanbun crate.
//! [`Buffer`] should be of interest to you.

use crossterm::{
    queue,
    style::{ResetColor, SetBackgroundColor, SetForegroundColor},
};
use std::{
    fmt,
    io::{self, stdout, BufWriter, Write},
};

/// Returned by [`size`] if querying the terminal size failed.
#[derive(Debug)]
pub struct TerminalSizeError;

impl fmt::Display for TerminalSizeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Failed to query terminal size")
    }
}

impl std::error::Error for TerminalSizeError {}

/// Returns the terminal's width and height.
///
/// # Examples
///
/// ```
/// if let Ok((width, height)) = hanbun::size() {
///     println!("Your terminal has {} cells!", width*height);
/// } else {
///     eprintln!("Failed to get terminal size!");
/// }
/// ```
///
/// # Errors
///
/// May return [`TerminalSizeError`] if the operation failed.
pub fn size() -> Result<(usize, usize), TerminalSizeError> {
    if let Ok((width, height)) = crossterm::terminal::size() {
        Ok((width as usize, height as usize))
    } else {
        Err(TerminalSizeError)
    }
}

/// See [this list](https://docs.rs/crossterm/0.19.0/crossterm/style/enum.Color.html) for all available colors.
pub type Color = crossterm::style::Color;

/// Represents a terminal cell. Every cell has two blocks.
#[derive(Debug, Clone)]
pub struct Cell {
    /// The upper block. It can be modified using [`Buffer::set`] and [`Buffer::color`].
    pub upper_block: Option<Option<Color>>,
    /// The lower block. It can be modified using [`Buffer::set`] and [`Buffer::color`].
    pub lower_block: Option<Option<Color>>,
    /// The character used if both [`Cell::upper_block`] and [`Cell::lower_block`] are [`None`].
    ///
    /// This character occupies the whole cell.
    pub char: Option<char>,
    /// A color for [`Cell::char`].
    pub char_color: Option<Color>,
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
    pub width: usize,
    pub height: usize,
    writer: BufWriter<io::Stdout>,
}

impl Buffer {
    /// Creates a new buffer of `width * height` cells filled with `char`.
    pub fn new(width: usize, height: usize, char: char) -> Buffer {
        Buffer {
            cells: vec![
                Cell {
                    upper_block: None,
                    lower_block: None,
                    char: Some(char),
                    char_color: None
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
            // NOTE: This can be improved after https://github.com/rust-lang/rust/issues/53667
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
            } else if let Some(char) = &cell.char {
                if let Some(color) = cell.char_color {
                    queue!(writer, SetForegroundColor(color)).unwrap();
                }

                write!(writer, "{}", char).unwrap();
                if cell.char_color.is_some() {
                    queue!(writer, ResetColor).unwrap();
                }
            } else {
                unreachable!();
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
            char: Some(char),
            char_color: None,
        })
    }

    /// Clears the buffer using `char` colored with `color`.
    pub fn colored_clear(&mut self, char: char, color: Color) {
        self.cells.fill(Cell {
            upper_block: None,
            lower_block: None,
            char: Some(char),
            char_color: Some(color),
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
            .unwrap_or_else(|| panic!("setting block at ({}, {}) (out of range)", x, y));

        if y % 2 == 0 {
            self.cells[position] = Cell {
                upper_block: Some(None),
                lower_block: current_cell.lower_block,
                char: None,
                char_color: None,
            };
        } else {
            self.cells[position] = Cell {
                upper_block: current_cell.upper_block,
                lower_block: Some(None),
                char: None,
                char_color: None,
            };
        }
    }

    /// Colors the cell at (`x`, `y`) with `color`.
    ///
    /// # Panics
    ///
    /// Panics if (`x`, `y`) is out of the buffer's range.
    pub fn color(&mut self, x: usize, y: usize, color: Color) {
        let position = x + self.width * (y / 2);
        let current_cell = &self
            .cells
            .get(position)
            .unwrap_or_else(|| panic!("coloring block at ({}, {}) (out of range)", x, y));

        if y % 2 == 0 {
            self.cells[position] = Cell {
                upper_block: Some(Some(color)),
                lower_block: current_cell.lower_block,
                char: None,
                char_color: None,
            };
        } else {
            self.cells[position] = Cell {
                upper_block: current_cell.upper_block,
                lower_block: Some(Some(color)),
                char: None,
                char_color: None,
            };
        }
    }

    /// Prints `string` to (`x`, `y`).
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
                .unwrap_or_else(|| panic!("printing at ({}, {}) (out of range)", x, y));

            *cell = Cell {
                upper_block: None,
                lower_block: None,
                char: Some(char),
                char_color: None,
            };
        }
    }

    /// Prints a colored `string` to (`x`, `y`) with `color`.
    ///
    /// # Panics
    ///
    /// Panics if (`x`, `y`) is out of the buffer's range.
    pub fn colored_print(&mut self, x: usize, y: usize, string: &str, color: Color) {
        let position = x + self.width * (y / 2);

        for (index, char) in string.chars().enumerate() {
            let cell = self
                .cells
                .get_mut(index + position)
                .unwrap_or_else(|| panic!("printing at ({}, {}) (out of range)", x, y));

            *cell = Cell {
                upper_block: None,
                lower_block: None,
                char: Some(char),
                char_color: Some(color),
            };
        }
    }
}
