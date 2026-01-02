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

pub fn render_sprites(
    mut commands: Commands,
    mut queue: ResMut<SpriteQueue>,
    // We query all existing pool sprites (mutable so we can move them)
    mut query: Query<(Entity, &mut Transform, &mut Sprite, &mut Visibility), With<ImmediateSprite>>,
) {
    let mut drawn_count = 0;

    // 1. MATCH EXISTING ENTITIES TO COMMANDS
    // We iterate through our pool of entities and the user's commands together
    for (command, (_entity, mut transform, mut sprite, mut vis)) in queue.0.iter().zip(query.iter_mut()) {

        // Update the entity to match the command
        transform.translation = command.position.extend(0.0);
        transform.scale = command.scale.extend(1.0);
        sprite.image = command.image.clone(); // In Bevy 0.17+ this might be sprite.texture
        sprite.color = command.color;

        // Make sure it's visible
        *vis = Visibility::Visible;

        drawn_count += 1;
    }

    // 2. SPAWN NEW ENTITIES (If we have more commands than entities)
    // If the user drew 100 sprites but we only have 50 in the pool, spawn 50 more.
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

    // 3. HIDE UNUSED ENTITIES
    // If user drew 5 sprites but we have 100 in the pool, hide the other 95.
    // (We don't despawn them, so we can reuse them next frame cheaply)
    for (_entity, _transform, _sprite, mut vis) in query.iter_mut().skip(drawn_count) {
        *vis = Visibility::Hidden;
    }

    // 4. CLEAR QUEUE
    queue.0.clear();
}