use hanbun::{self, Color};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use std::process;

#[derive(Clone, PartialEq)]
struct Cursor {
    x: usize,
    y: usize,
}

const GREATER_THAN_SIGN: &str = "WW
  WW
    W
  WW
WW";

impl Cursor {
    const CHARACTER_SIZE: usize = 6;
    const SHAPE: &'static str = "WWW
 W
 W
 W
WWW";

    fn advance(&mut self) {
        self.x += Self::CHARACTER_SIZE;
    }

    fn revert(&mut self) {
        self.x -= Self::CHARACTER_SIZE;
    }

    fn next_line(&mut self) {
        self.y += Self::CHARACTER_SIZE;
        self.x = 0;
    }

    fn previous_line(&mut self) {
        self.y -= Self::CHARACTER_SIZE;
        self.x = Self::CHARACTER_SIZE;
    }

    fn draw(&self, buffer: &mut hanbun::Buffer) {
        w_to_half_block(buffer, self, Cursor::SHAPE, |x, y, buffer| {
            buffer.color(x, y, Color::DarkGrey);
        });
    }
}

fn w_to_half_block(
    buffer: &mut hanbun::Buffer,
    cursor: &Cursor,
    string: &str,
    function: fn(usize, usize, &mut hanbun::Buffer),
) {
    let mut x = 0;
    let mut y = 0;
    for line in string.lines() {
        for char in line.chars() {
            if char == 'W' {
                function(cursor.x + x, cursor.y + y, buffer);
            }
            x += 1;
        }
        y += 1;
        x = 0;
    }
}

fn clear_space(buffer: &mut hanbun::Buffer, cleared_spaces: &mut Vec<Cursor>, cursor: &Cursor) {
    for y in 0..Cursor::CHARACTER_SIZE {
        // The space's brightness depends on how often the space has been cleared
        let count = cleared_spaces
            .iter()
            .filter(|cleared_space| *cleared_space == cursor)
            .count();
        let clear_character = match count {
            0 => "     ",
            1 => "░░░░░",
            2 => "▒▒▒▒▒",
            3 | _ => "▓▓▓▓▓",
        };
        buffer.colored_print(cursor.x, cursor.y + y, clear_character, Color::DarkGrey);
    }
}

fn main() {
    let mut buffer = match hanbun::size() {
        Ok((width, height)) => {
            if height < 3 {
                eprintln!("Terminal height needs to be >=3.");
                process::exit(1);
            }
            hanbun::Buffer::new(width, height, ' ')
        }
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };

    let mut cursor = Cursor { x: 0, y: 0 };

    let mut cleared_spaces = Vec::<Cursor>::new();

    w_to_half_block(&mut buffer, &cursor, GREATER_THAN_SIGN, |x, y, buffer| {
        buffer.color(x, y, Color::White);
    });
    cursor.advance();

    buffer.print(12, 0, "Press  type  to");
    buffer.print(12, 2, "AIUEO  and  quit.");
    buffer.print(12, 4, " to    ESC");

    loop {
        cursor.draw(&mut buffer);
        buffer.draw();

        // We need raw mode while reading a key
        enable_raw_mode().unwrap();
        if let Event::Key(event) = event::read().unwrap() {
            disable_raw_mode().unwrap();

            let katakana = match event.code {
                KeyCode::Char('a') => {
                    "WWWWW
    W
 W W
 W
W"
                }

                KeyCode::Char('i') => {
                    "    W
  WW
WWW
  W
  W"
                }

                KeyCode::Char('u') => {
                    "  W
WWWWW
W   W
   W
  W
"
                }

                KeyCode::Char('e') => {
                    "WWWWW
  W
  W
  W
WWWWW
"
                }

                KeyCode::Char('o') => {
                    "   W
WWWWW
  WW
 W W
W WW"
                }
                KeyCode::Char(' ') => {
                    clear_space(&mut buffer, &mut cleared_spaces, &cursor);

                    cursor.advance();
                    continue;
                }
                KeyCode::Backspace => {
                    if cursor.x == Cursor::CHARACTER_SIZE {
                        if cursor.y != 0 {
                            clear_space(&mut buffer, &mut cleared_spaces, &cursor);
                            cursor.revert();
                            clear_space(&mut buffer, &mut cleared_spaces, &cursor);

                            cursor.previous_line();
                        }
                    } else {
                        clear_space(&mut buffer, &mut cleared_spaces, &cursor);
                        if cursor.x != Cursor::CHARACTER_SIZE {
                            cursor.revert();
                        }

                        // We only push after this clear_space call
                        // because we want the brightness to increase only for this case
                        cleared_spaces.push(cursor.clone());
                        clear_space(&mut buffer, &mut cleared_spaces, &cursor);
                    }
                    continue;
                }
                KeyCode::Enter => {
                    clear_space(&mut buffer, &mut cleared_spaces, &cursor);
                    cursor.next_line();
                    w_to_half_block(&mut buffer, &cursor, GREATER_THAN_SIGN, |x, y, buffer| {
                        buffer.color(x, y, Color::White);
                    });
                    cursor.advance();
                    continue;
                }
                KeyCode::Esc => process::exit(0),
                _ => {
                    "  WW
   W
  WW

  W "
                }
            };
            clear_space(&mut buffer, &mut cleared_spaces, &cursor);
            w_to_half_block(&mut buffer, &cursor, katakana, |x, y, buffer| {
                buffer.set(x, y);
            });
            cursor.advance();
        }
    }
}
