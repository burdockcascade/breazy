use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;
use crate::text::{TextCommand, TextQueue};

pub struct Context<'a> {
    pub time: &'a Time,
}

pub struct DrawContext<'a, 'w, 's> {
    pub painter: &'a mut ShapePainter<'w, 's>,
    pub text_queue: &'a mut ResMut<'w, TextQueue>,
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

}
