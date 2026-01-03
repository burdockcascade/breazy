mod context;
mod text;
mod sprite;
mod audio;

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_vector_shapes::prelude::*;
use crate::audio::{play_audio, AudioQueue};
pub use crate::context::{Context, DrawContext};
use crate::context::{AudioContext, InputContext};
use crate::sprite::{render_sprites, SpriteQueue};
use crate::text::{render_text, TextQueue};

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
    // Graphics
    pub painter: ShapePainter<'w, 's>,

    // Queues
    pub text_queue: ResMut<'w, TextQueue>,
    pub sprite_queue: ResMut<'w, SpriteQueue>,
    pub audio_queue: ResMut<'w, AudioQueue>,

    // Core
    pub time: Res<'w, Time>,
    pub asset_server: Res<'w, AssetServer>,

    // Input
    pub keys: Res<'w, ButtonInput<KeyCode>>,
    pub mouse_buttons: Res<'w, ButtonInput<MouseButton>>,

    // Window / Camera (for mouse calculation)
    pub q_window: Query<'w, 's, &'static Window, With<PrimaryWindow>>,
    pub q_camera: Query<'w, 's, (&'static Camera, &'static GlobalTransform), With<Camera2d>>,
}

fn internal_game_loop<G: Game>(mut game: NonSendMut<G>, mut engine: EngineContext, mut state: Local<InternalState>) {

    let mut cursor_world_pos = Vec2::ZERO;
    if let (Ok(window), Ok((camera, camera_transform))) = (engine.q_window.single(), engine.q_camera.single()) {
        if let Some(screen_pos) = window.cursor_position() {
            // Convert Screen (Top-Left) -> World (Center)
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, screen_pos) {
                cursor_world_pos = world_pos;
            }
        }
    }

    // --- UPDATE STEP ---
    {
        let audio_ctx = AudioContext {
            queue: &mut engine.audio_queue,
            asset_server: &engine.asset_server,
        };

        let input_ctx = InputContext {
            keys: &engine.keys,
            mouse_buttons: &engine.mouse_buttons,
            cursor_world_pos,
        };

        let mut ctx = Context {
            time: &engine.time,
            audio: audio_ctx,
            asset_server: &engine.asset_server,
            input: input_ctx,
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
            painter: &mut engine.painter,
            text_queue: &mut engine.text_queue,
            sprite_queue: &mut engine.sprite_queue,
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
        .add_plugins(Shape2dPlugin::default())
        .insert_resource(TextQueue::default())
        .insert_resource(SpriteQueue::default())
        .insert_resource(AudioQueue::default())
        .insert_non_send_resource(game)
        .add_systems(Startup, setup_camera)
        .add_systems(Update, (
            internal_game_loop::<G>,
            render_text,
            render_sprites,
            play_audio
        ).chain())
        .run();
}