use bevy::prelude::*;

// 1. THE COMMAND
#[derive(Clone)]
pub struct TextCommand {
    pub text: String,
    pub position: Vec2,
    pub size: f32,
    pub color: Color,
}

// 2. THE QUEUE
#[derive(Resource, Default)]
pub struct TextQueue(pub Vec<TextCommand>);

// 3. THE MARKER
#[derive(Component)]
pub struct ImmediateText;

pub fn render_text(
    mut commands: Commands,
    mut queue: ResMut<TextQueue>,
    // Query existing text entities in our pool
    // We need 'Entity', 'Transform', 'Text', 'TextFont', 'TextColor', 'Visibility'
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut Text,
        &mut TextFont,
        &mut TextColor,
        &mut Visibility
    ), With<ImmediateText>>,
) {
    let mut drawn_count = 0;

    // 1. MATCH EXISTING ENTITIES
    for (command, (_entity, mut transform, mut text, mut font, mut color, mut vis)) in queue.0.iter().zip(query.iter_mut()) {

        // Update content
        text.0 = command.text.clone(); // The string content

        // Update position
        transform.translation = Vec3::new(command.position.x, command.position.y, 0.0);

        // Update style
        font.font_size = command.size;
        color.0 = command.color;

        *vis = Visibility::Visible;
        drawn_count += 1;
    }

    // 2. SPAWN NEW ENTITIES
    if queue.0.len() > drawn_count {
        for command in queue.0.iter().skip(drawn_count) {
            commands.spawn((
                Text2d::new(command.text.clone()),
                Transform::from_xyz(command.position.x, command.position.y, 0.0),
                TextFont {
                    font_size: command.size,
                    ..default()
                },
                TextColor(command.color),
                ImmediateText,
            ));
        }
    }

    // 3. HIDE UNUSED
    for (_entity, _transform, _text, _font, _color, mut vis) in query.iter_mut().skip(drawn_count) {
        *vis = Visibility::Hidden;
    }

    // 4. CLEAR
    queue.0.clear();
}