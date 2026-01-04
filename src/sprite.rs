use bevy::ecs::query::QueryData;
use bevy::prelude::*;

#[derive(Clone)]
pub struct SpriteCommand {
    pub image: Handle<Image>,
    pub position: Vec2,
    pub scale: Vec2,
    pub color: Color,
}

#[derive(Resource, Default)]
pub struct SpriteQueue(pub(crate) Vec<SpriteCommand>);

#[derive(Component)]
pub struct ImmediateSprite;

#[derive(QueryData)]
#[query_data(mutable)]
pub struct SpriteItem {
    pub entity: Entity,
    pub transform: &'static mut Transform,
    pub sprite: &'static mut Sprite,
    pub visibility: &'static mut  Visibility,
}

pub fn render_sprites( mut commands: Commands,  mut queue: ResMut<SpriteQueue>, mut query: Query<SpriteItem, With<ImmediateSprite>>) {
    let mut drawn_count = 0;

    // We iterate through our pool of entities and the user's commands together
    for (command, mut sprite_item) in queue.0.iter().zip(query.iter_mut()) {

        // Update the entity to match the command
        sprite_item.transform.translation = command.position.extend(0.0);
        sprite_item.transform.scale = command.scale.extend(1.0);
        sprite_item.sprite.image = command.image.clone();
        sprite_item.sprite.color = command.color;

        // Make sure it's visible
        *sprite_item.visibility = Visibility::Visible;

        drawn_count += 1;
    }

    // If we have more commands than entities in the pool, spawn new ones
    if queue.0.len() > drawn_count {
        for command in queue.0.iter().skip(drawn_count) {
            commands.spawn((
                Sprite {
                    image: command.image.clone(),
                    color: command.color,
                    ..default()
                },
                Transform {
                    translation: command.position.extend(0.0),
                    scale: command.scale.extend(1.0),
                    ..default()
                },
                // Add Marker so we can find it next frame
                ImmediateSprite,
            ));
        }
    }

    // Hide any remaining entities in the pool that were not used this frame
    for mut sprite_item in query.iter_mut().skip(drawn_count) {
        *sprite_item.visibility = Visibility::Hidden;
    }

    // Clear the queue for the next frame
    queue.0.clear();
}