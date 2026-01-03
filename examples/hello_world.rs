use bevy::color::palettes::css;
use breazy::prelude::*;

struct MyGame {
    time: f32,
}

impl Game for MyGame {
    fn update(&mut self, _ctx: &mut Context) {
        self.time += 0.01;
    }

    fn draw(&mut self, ctx: &mut DrawContext) {


        // Draw a Circle that moves
        let x_pos = self.time.sin() * 200.0;
        ctx.circle(x_pos, 0.0, 60.0, Color::from(css::RED));

        // Draw a static Rectangle
        ctx.rect(0.0, -200.0, 400.0, 50.0, Color::from(css::BLUE_VIOLET));
        ctx.text("Hello, World!", -0.0, -200.0);

        // Draw a Ring that pulses
        let ring_radius = 100.0 + (self.time * 5.0).sin() * 25.0;
        ctx.ring(x_pos, 0.0, ring_radius, 10.0, Color::from(css::YELLOW));
    }
}

fn main() {
    // 1. Configure
    let config = AppConfig {
        title: "Hello Window".to_string(),
        width: 1280,
        height: 720,
    };

    let my_game = MyGame { time: 0.0 };

    // 2. Run
    run(config, my_game);
}