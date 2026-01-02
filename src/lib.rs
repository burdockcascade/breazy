mod context;

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub use crate::context::{Context, DrawContext};

pub mod prelude {
    pub use crate::{run, AppConfig, Game};
    pub use crate::{Context, DrawContext};
    pub use bevy::prelude::*;
}

pub struct AppConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            title: "My Game".to_string(),
            width: 800,
            height: 600,
        }
    }
}

pub trait Game: Send + Sync + 'static {
    fn init(&mut self, _ctx: &mut Context) {}
    fn update(&mut self, ctx: &mut Context);
    fn draw(&mut self, ctx: &mut DrawContext);
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Default)]
struct InternalState { initialized: bool }

#[derive(SystemParam)]
pub struct EngineContext<'w, 's> {

    // Core
    pub time: Res<'w, Time>,

    // Window / Camera (for mouse calculation)
    pub q_window: Query<'w, 's, &'static Window, With<PrimaryWindow>>,
    pub q_camera: Query<'w, 's, (&'static Camera, &'static GlobalTransform), With<Camera2d>>,
}

fn internal_game_loop<G: Game>(mut game: NonSendMut<G>, engine: EngineContext, mut state: Local<InternalState>) {

    // --- UPDATE STEP ---
    {

        let mut ctx = Context {
            time: &engine.time,
        };

        if !state.initialized {
            game.init(&mut ctx);
            state.initialized = true;
        }

        game.update(&mut ctx);
    }

    // --- DRAW STEP ---
    {
        let mut draw_ctx = DrawContext {
            time: &engine.time,
        };
        game.draw(&mut draw_ctx);
    }
}

pub fn run<G: Game>(config: AppConfig, game: G) {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: config.title,
                resolution: (config.width, config.height).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_non_send_resource(game)
        .add_systems(Startup, setup_camera)
        .add_systems(Update, (
            internal_game_loop::<G>,
        ).chain())
        .run();
}