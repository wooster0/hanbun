<h1 align="center">
  半分
</h1>

<h5 align="center">
  hanbun is a library for drawing half blocks (<code>▀</code> and <code>▄</code>) to the terminal, which allows rendering of various graphics.
</h5>

## Example

Here is an example that makes use of common features
such as creating a buffer, setting colored half blocks, printing text
and finally drawing the buffer to the screen.

```rust
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

    let width = lines.iter().map(|line| line.len()).max().unwrap();
    let height = lines.len() / 2 + 2;
    // Here we store the state of each cell
    let mut buffer = hanbun::Buffer::new(width, height, ' ');

    let mut x = 0;
    let mut y = 0;
    for line in &lines {
        for char in line.chars() {
            // We set a colored half block for each W we find
            if char == 'W' {
                buffer.color(x, y, Color::Green);
            }
            x += 1;
        }
        y += 1;
        x = 0;
    }

    // Add some centered text to the bottom
    let text = "hanbun";
    buffer.print(width / 2 - text.len() / 2, height + 5, text);

    // Actually display what we've drawn
    buffer.draw();
}
```

The result:

![result](https://user-images.githubusercontent.com/35064754/108411280-b96b3000-7228-11eb-9e06-41b8f634a195.png)

Results may vary depending on the terminal.

Make sure to check out the other examples too!
You can clone this repository and run examples
like the calculator using `cargo run --example calculator`.

## Footnotes

This is my first Rust library to make some crate development experience.
As of now, the library is useable but if you do repeated drawing you might experience some tearing. On the bright side you get that nostalgic feeling of old displays.
