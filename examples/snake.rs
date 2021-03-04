use hanbun::{self, Color};

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use std::{process, result, thread, time};

use crossterm::{
    event::{poll, read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

#[derive(Clone, Copy, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

const WIDTH: usize = 50;
const HEIGHT: usize = 50;

fn main() -> result::Result<(), String> {
    let mut buffer;
    if let Ok((width, height)) = hanbun::size() {
        if width <= WIDTH || height <= HEIGHT / 2 {
            return Err(format!(
                "Terminal width and height need to be >={} and >={} respectively.",
                WIDTH,
                HEIGHT / 2
            ));
        }
        buffer = hanbun::Buffer::new(width, height, ' ');
    } else {
        return Err(String::from("Unable to retrieve terminal size"));
    }

    let mut rng = SmallRng::from_entropy();

    let start_x = rng.gen_range(10..WIDTH - 10);
    let start_y = rng.gen_range(10..HEIGHT - 10);
    let mut parts = vec![Position {
        x: start_x,
        y: start_y,
    }];

    let mut food_position = spawn_food(&mut buffer, WIDTH, HEIGHT, &mut rng);
    let mut direction = match rng.gen_range(0..4) {
        0 => Direction::Up,
        1 => Direction::Down,
        2 => Direction::Left,
        3 => Direction::Right,
        _ => unreachable!(),
    };
    let mut score = 0;
    let mut delay = 100;

    loop {
        draw_border(&mut buffer);

        let head = parts[0];
        if head == food_position {
            food_position = spawn_food(&mut buffer, WIDTH, HEIGHT, &mut rng);
            parts.push(head);

            // Speed up
            if delay != 50 {
                delay -= 10;
            }

            score += 1;
        }

        center(&mut buffer, "Controls:", HEIGHT / 2 - 6);
        center(
            &mut buffer,
            "Move: W, A, S, D, Speed Boost: hold",
            HEIGHT / 2 - 4,
        );

        for part in &parts {
            buffer.color(part.x, part.y, Color::Green);
        }

        center(&mut buffer, &score.to_string(), 0);

        // This has to come last so that it's visible in every case
        buffer.color(food_position.x, food_position.y, Color::Red);

        buffer.draw();
        buffer.clear(' ');

        handle_input(&mut direction, delay);

        // `game_over` is only called below this point
        // because at this point the buffer is cleared

        r#move(&mut buffer, &mut parts, &direction, score);

        let head = parts[0];
        for part in &parts[1..] {
            if head == *part {
                game_over(&mut buffer, score);
            }
        }
    }
}

fn r#move(
    buffer: &mut hanbun::Buffer,
    parts: &mut Vec<Position>,
    direction: &Direction,
    score: usize,
) {
    for index in (1..parts.len()).rev() {
        parts[index] = parts[index - 1];
    }

    let head = parts[0];
    match direction {
        Direction::Up => {
            if head.y == 1 {
                game_over(buffer, score);
            } else {
                parts[0].y -= 1;
            }
        }
        Direction::Down => {
            if head.y == WIDTH - 1 {
                game_over(buffer, score)
            } else {
                parts[0].y += 1;
            }
        }
        Direction::Left => {
            if head.x == 1 {
                game_over(buffer, score);
            } else {
                parts[0].x -= 1;
            }
        }
        Direction::Right => {
            if head.x == WIDTH - 1 {
                game_over(buffer, score);
            } else {
                parts[0].x += 1;
            }
        }
    }
}

fn draw_border(buffer: &mut hanbun::Buffer) {
    // -----
    //
    // -----
    for index in 1..WIDTH {
        buffer.set(index, 0);
        buffer.set(index, HEIGHT);
    }
    // |   |
    // |   |
    // |   |
    for index in 1..HEIGHT {
        buffer.set(0, index);
        buffer.set(WIDTH, index);
    }
    // +---+
    // |   |
    // +---+
}

fn spawn_food(
    buffer: &mut hanbun::Buffer,
    width: usize,
    height: usize,
    rng: &mut SmallRng,
) -> Position {
    let x = rng.gen_range(1..width);
    let y = rng.gen_range(1..height);
    let position = Position { x, y };

    buffer.color(position.x, position.y, Color::Red);

    position
}

fn center(buffer: &mut hanbun::Buffer, message: &str, alignment: usize) {
    buffer.print(
        WIDTH / 2 - message.len() / 2,
        HEIGHT / 2 + alignment,
        message,
    );
}

fn game_over(buffer: &mut hanbun::Buffer, score: usize) {
    center(buffer, "GAME OVER", 0);
    center(buffer, &format!("Score: {}", score), 4);
    buffer.draw();
    thread::sleep(time::Duration::from_secs(1));
    process::exit(0);
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn handle_input(direction: &mut Direction, delay: u64) {
    enable_raw_mode().unwrap();

    if poll(time::Duration::from_millis(delay)).unwrap() {
        // Next `read` is guaranteed not to block if `poll` returns `Ok(true)`
        if let Event::Key(event) = read().unwrap() {
            match event.code {
                KeyCode::Char('w') => {
                    if *direction != Direction::Down {
                        *direction = Direction::Up;
                    }
                }
                KeyCode::Char('s') => {
                    if *direction != Direction::Up {
                        *direction = Direction::Down;
                    }
                }
                KeyCode::Char('a') => {
                    if *direction != Direction::Right {
                        *direction = Direction::Left;
                    }
                }
                KeyCode::Char('d') => {
                    if *direction != Direction::Left {
                        *direction = Direction::Right;
                    }
                }
                KeyCode::Esc => {
                    disable_raw_mode().unwrap();
                    process::exit(0);
                }
                _ => (),
            }
        }
    }

    disable_raw_mode().unwrap();
}
