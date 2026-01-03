use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;
use crate::sprite::{SpriteCommand, SpriteQueue};
use crate::text::{TextCommand, TextQueue};
use crate::audio::*;

pub struct Context<'a> {
    pub asset_server: &'a AssetServer,
    pub audio: AudioContext<'a>,
    pub input: InputContext<'a>,
    pub time: &'a Time,
}

impl<'a> Context<'a> {
    /// Load an image from the "assets" folder
    pub fn load_image(&self, path: &str) -> Handle<Image> {
        self.asset_server.load(path.to_owned())
    }
}

pub struct DrawContext<'a, 'w, 's> {
    pub painter: &'a mut ShapePainter<'w, 's>,
    pub text_queue: &'a mut ResMut<'w, TextQueue>,
    pub sprite_queue: &'a mut ResMut<'w, SpriteQueue>,
    pub time: &'a Time,
}

impl<'a, 'w, 's> DrawContext<'a, 'w, 's> {

    /// Draw a filled circle
    pub fn circle(&mut self, x: f32, y: f32, radius: f32, color: Color) {
        self.painter.set_translation(Vec3::new(x, y, 0.0));
        self.painter.color = color;
        self.painter.circle(radius);
    }

    /// Draw a filled rectangle
    pub fn rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        self.painter.set_translation(Vec3::new(x, y, 0.0));
        self.painter.color = color;
        self.painter.rect(Vec2::new(w, h));
    }

    /// Draw a hollow ring (just to show versatility)
    pub fn ring(&mut self, x: f32, y: f32, radius: f32, thickness: f32, color: Color) {
        self.painter.set_translation(Vec3::new(x, y, 0.0));
        self.painter.color = color;
        self.painter.hollow = true;
        self.painter.thickness = thickness;
        self.painter.circle(radius);
        self.painter.hollow = false;
    }

    /// Draw simple text using the default font
    pub fn text(&mut self, text: &str, x: f32, y: f32) {
        self.text_queue.0.push(TextCommand {
            text: text.to_string(),
            position: Vec2::new(x, y),
            size: 20.0,
            color: Color::WHITE,
        });
    }

    /// Draw text with size and color
    pub fn text_ext(&mut self, text: &str, x: f32, y: f32, size: f32, color: Color) {
        self.text_queue.0.push(TextCommand {
            text: text.to_string(),
            position: Vec2::new(x, y),
            size,
            color,
        });
    }

    pub fn sprite(&mut self, image: &Handle<Image>, x: f32, y: f32) {
        self.sprite_queue.0.push(SpriteCommand {
            image: image.clone(),
            position: Vec2::new(x, y),
            scale: Vec2::ONE,
            color: Color::WHITE,
        });
    }

    /// Draw a scaled or tinted sprite
    pub fn sprite_ext(&mut self, image: &Handle<Image>, x: f32, y: f32, scale: f32, color: Color) {
        self.sprite_queue.0.push(SpriteCommand {
            image: image.clone(),
            position: Vec2::new(x, y),
            scale: Vec2::splat(scale),
            color,
        });
    }

}

pub struct AudioContext<'a> {
    pub(crate) queue: &'a mut AudioQueue,
    pub(crate) asset_server: &'a AssetServer,
}

impl<'a> AudioContext<'a> {
    /// Play a sound file once (Fire and Forget)
    pub fn play(&mut self, path: &str) {
        let handle = self.asset_server.load(path.to_owned());
        self.queue.0.push(AudioCommand {
            sound: handle,
            volume: 1.0,
        });
    }

    /// Play a sound with specific volume (0.0 to 1.0)
    pub fn play_vol(&mut self, path: &str, volume: f32) {
        let handle = self.asset_server.load(path.to_owned());
        self.queue.0.push(AudioCommand {
            sound: handle,
            volume,
        });
    }
}

pub struct InputContext<'a> {
    pub(crate) keys: &'a ButtonInput<KeyCode>,
    pub(crate) mouse_buttons: &'a ButtonInput<MouseButton>,
    pub(crate) cursor_world_pos: Vec2, // We calculate this once per frame in lib.rs
}

impl<'a> InputContext<'a> {
    // --- KEYBOARD ---

    /// Returns true while the key is held down
    pub fn key_down(&self, key: KeyCode) -> bool {
        self.keys.pressed(key)
    }

    /// Returns true only on the frame the key was pressed
    pub fn key_pressed(&self, key: KeyCode) -> bool {
        self.keys.just_pressed(key)
    }

    /// Returns true only on the frame the key was released
    pub fn key_released(&self, key: KeyCode) -> bool {
        self.keys.just_released(key)
    }

    // --- MOUSE ---

    /// Returns the mouse position in World Space (0,0 is center of screen)
    pub fn mouse_pos(&self) -> Vec2 {
        self.cursor_world_pos
    }

    pub fn mouse_down(&self, button: MouseButton) -> bool {
        self.mouse_buttons.pressed(button)
    }

    pub fn mouse_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons.just_pressed(button)
    }
}
