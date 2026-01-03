use bevy::prelude::*;

#[derive(Clone)]
pub struct AudioCommand {
    pub sound: Handle<AudioSource>,
    pub volume: f32,
}

#[derive(Resource, Default)]
pub struct AudioQueue(pub Vec<AudioCommand>);

pub fn play_audio(mut commands: Commands, mut queue: ResMut<AudioQueue>) {
    for cmd in queue.0.drain(..) {
        commands.spawn((
            AudioPlayer(cmd.sound),
            PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: bevy::audio::Volume::Linear(cmd.volume),
                ..default()
            },
        ));
    }
}