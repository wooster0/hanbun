use hanbun;

use crossterm::{
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use meval;

use std::process;

#[derive(Debug)]
struct Button {
    x: usize,
    y: usize,
    digit: usize,
}

// Generates a grid of cells (not to be confused with the terminal's cells).
fn grid(
    buffer: &mut hanbun::Buffer,
    base_x: usize,
    base_y: usize,
    width: usize,
    height: usize,
    cell_size: usize,
) -> Vec<Button> {
    let mut index = 0;
    for x in 0..=width * cell_size + width {
        // Draw the horizontal line
        buffer.set(base_x + x, base_y);
        if index > cell_size {
            for y in 0..height * cell_size + height {
                // Draw the vertical lines
                buffer.set(base_x + x, base_y + y);
            }
            index = 0;
        }
        index += 1;
    }

    index = 0;
    for y in 0..=height * cell_size + height {
        // Draw the vertical line
        buffer.set(base_x, base_y + y);
        if index > cell_size {
            for x in 0..width * cell_size + width {
                // Draw the horizontal lines
                buffer.set(base_x + x + 1, base_y + y);
            }
            index = 0;
        }
        index += 1;
    }

    let mut buttons = Vec::<Button>::with_capacity(width * height);
    for x in 0..width {
        for y in 0..height {
            let x_center = base_x + x + x * cell_size + cell_size / 2 + 1;
            let y_center = base_y + y + y * cell_size + cell_size / 2 + 1;
            buttons.push(Button {
                x: x_center,
                y: y_center,
                digit: 0,
            });
        }
    }

    buttons
}

fn is_operator(input: &str) -> bool {
    input.ends_with('+') || input.ends_with('-')
}

fn main() -> std::result::Result<(), ()> {
    let size = hanbun::size();
    let mut buffer;
    if let Ok((width, height)) = size {
        let required_width = 25;
        let required_height = 21;
        if width <= required_width || height <= required_height {
            eprintln!(
                "Terminal width and height need to be >={} and >={} respectively.",
                required_width, required_height
            );
            process::exit(1);
        }
        buffer = hanbun::Buffer::new(width as usize, height as usize, ' ');
    } else {
        eprintln!("Unable to get terminal size");
        process::exit(1);
    };

    println!("Input: W, A, S, D and Enter\nExit: ESC\nPress any of these keys to start!");

    let mut cursor_x = 0;
    let mut cursor_y = 0;
    let mut input = String::new();
    let mut result = String::new();
    loop {
        // Because we can only have squares,
        // the first grid (the display, which is only a single square)
        // is overlapping with the second grid (the buttons)
        // to make it rectangular.
        let base_x = 0;
        let base_y = 0;
        let width = 1;
        let height = 1;
        let cell_size = 23;
        grid(&mut buffer, base_x, base_y, width, height, cell_size);

        let base_y = 8;
        let width = 3;
        let height = 4;
        let cell_size = 7;
        let buttons = grid(&mut buffer, base_x, base_y, width, height, cell_size);

        buffer.print(buttons[0].x, buttons[0].y, "1");
        buffer.print(buttons[1].x, buttons[1].y, "4");
        buffer.print(buttons[2].x, buttons[2].y, "7");
        buffer.print(buttons[3].x, buttons[3].y, "+");

        buffer.print(buttons[4].x, buttons[4].y, "2");
        buffer.print(buttons[5].x, buttons[5].y, "5");
        buffer.print(buttons[6].x, buttons[6].y, "8");
        buffer.print(buttons[7].x, buttons[7].y, "0");

        buffer.print(buttons[8].x, buttons[8].y, "3");
        buffer.print(buttons[9].x, buttons[9].y, "6");
        buffer.print(buttons[10].x, buttons[10].y, "9");
        buffer.print(buttons[11].x, buttons[11].y, "-");

        enable_raw_mode().unwrap();
        let key = read().unwrap();
        disable_raw_mode().unwrap();

        match key {
            Event::Key(event) => match event.code {
                KeyCode::Char('w') => {
                    if cursor_y == 0 {
                        cursor_y = height - 1;
                    } else {
                        cursor_y -= 1
                    }
                }
                KeyCode::Char('a') => {
                    if cursor_x == 0 {
                        cursor_x = width - 1;
                    } else {
                        cursor_x -= 1
                    }
                }
                KeyCode::Char('s') => {
                    if cursor_y == height - 1 {
                        cursor_y = 0;
                    } else {
                        cursor_y += 1
                    }
                }
                KeyCode::Char('d') => {
                    if cursor_x == width - 1 {
                        cursor_x = 0;
                    } else {
                        cursor_x += 1
                    }
                }
                KeyCode::Enter => {
                    for y in 0..height {
                        for x in 0..width {
                            if cursor_x == x && cursor_y == y {
                                // Map the buttons
                                let char = match (x, y) {
                                    (0, 0) => '1',
                                    (1, 0) => '2',
                                    (2, 0) => '3',
                                    (0, 1) => '4',
                                    (1, 1) => '5',
                                    (2, 1) => '6',
                                    (0, 2) => '7',
                                    (1, 2) => '8',
                                    (2, 2) => '9',
                                    (0, 3) => {
                                        if is_operator(&input) {
                                            break;
                                        }
                                        '+'
                                    }
                                    (1, 3) => '0',
                                    (2, 3) => {
                                        if is_operator(&input) {
                                            break;
                                        }
                                        '-'
                                    }
                                    _ => '1',
                                };
                                input.push(char);

                                if !is_operator(&input) {
                                    result = meval::eval_str(&input).unwrap().to_string();
                                }
                            }
                        }
                    }
                }
                KeyCode::Esc => break,
                _ => continue,
            },
            _ => continue,
        }

        buffer.print(2, 2, &input);
        buffer.print(2, 4, &result);

        // Fill the selected button.
        for y in 0..height {
            for x in 0..width {
                if cursor_x == x && cursor_y == y {
                    for i1 in 0..cell_size {
                        for i2 in 0..cell_size {
                            buffer.set(
                                i1 + 1 + base_x + x + x * cell_size,
                                i2 + 1 + base_y + y + y * cell_size,
                            );
                        }
                    }
                }
            }
        }

        buffer.draw();
        buffer.clear(' ');
    }

    Ok(())
}
