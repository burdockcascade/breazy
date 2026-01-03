# Breasy
Breazy is an open-source framework designed to make game development in Rust accessible, fast, and fun.

Built on top of the industry-leading Bevy Engine, Breazy strips away the boilerplate of Entity Component Systems (ECS). Instead of managing queries, plugins, and systems, you get a single Context and a familiar game loop. It combines the raw performance of Bevy with the simplicity of creative coding tools like Processing or Raylib.

Whether you are building a prototype, a game jam entry, or learning Rust for the first time, Breazy lets you draw shapes, play sounds, and handle input in just a few lines of code.

## Example
```rust
use breazy::prelude::*;

struct MyGame {
    time: f32,
}

impl Game for MyGame {
    fn update(&mut self, ctx: &mut Context) {
        self.time = ctx.time.elapsed_secs();
    }

    fn draw(&mut self, ctx: &mut DrawContext) {

        // Draw a Circle that moves
        let x_pos = self.time.sin() * 200.0;
        ctx.circle(x_pos, 0.0, 60.0, Color::from(RED));

        // Draw a static Rectangle
        ctx.rect(0.0, -200.0, 400.0, 50.0, Color::from(BLUE_VIOLET));
        ctx.text("Hello, World!", -0.0, -200.0);

        // Draw a Ring that pulses
        let ring_radius = 100.0 + (self.time * 5.0).sin() * 25.0;
        ctx.ring(x_pos, 0.0, ring_radius, 10.0, Color::from(YELLOW));
    }
}

fn main() {
    let config = AppConfig {
        title: "Hello Window".to_string(),
        width: 1280,
        height: 720,
    };
    let my_game = MyGame { time: 0.0 };
    run(config, my_game);
}
```