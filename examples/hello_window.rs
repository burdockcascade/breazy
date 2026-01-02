use breazy::prelude::*;

struct MyGame;

impl Game for MyGame {
    fn init(&mut self, _ctx: &mut Context) {
    }

    fn update(&mut self, _ctx: &mut Context) {
    }

    fn draw(&mut self, _ctx: &mut DrawContext) {
    }
}

fn main() {
    // 1. Configure
    let config = AppConfig {
        title: "Hello Window".to_string(),
        width: 1280,
        height: 720,
    };

    // 2. Run
    run(config, MyGame);
}