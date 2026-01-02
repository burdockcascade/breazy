use bevy::prelude::*;

pub struct Context<'a> {
    pub time: &'a Time,
}

pub struct DrawContext<'a> {
    pub time: &'a Time,
}

