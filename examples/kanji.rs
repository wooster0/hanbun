use hanbun::{self, Color};

fn main() {
    // Let's draw these two kanji on the screen using half blocks!
    let lines = [
        " W   W   W      W W",
        "  W  W  W      W   W",
        "     W        W     W",
        " WWWWWWWWW   W       W",
        "     W      W         W",
        "     W       WWWWWWWWW",
        "WWWWWWWWWWW     W    W",
        "     W          W    W",
        "     W          W    W",
        "     W         W  W W",
        "     W       WW    W",
    ];

    // Here we store the state of each cell
    let width = lines.iter().map(|line| line.len()).max().unwrap();
    let height = lines.len() / 2 + 2;
    let mut buffer = hanbun::Buffer::new(width, height, ' ');

    let mut x = 0;
    let mut y = 0;
    for line in &lines {
        for char in line.chars() {
            // We place a colored half block for each W we find
            if char == 'W' {
                buffer.color(x, y, Color::Green);
            }
            // Advance to the right
            x += 1;
        }
        // New line
        y += 1;
        x = 0;
    }

    // Add some centered text to the bottom
    let text = "hanbun";
    buffer.print(width / 2 - text.len() / 2, height + 5, text);

    // Actually display what we've drawn
    buffer.draw();
}
