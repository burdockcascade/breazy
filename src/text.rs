use bevy::ecs::query::QueryData;
use bevy::prelude::*;
use bevy::sprite::Anchor;

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

#[derive(QueryData)]
#[query_data(mutable)]
pub struct TextItem {
    pub entity: Entity,
    pub transform: &'static mut Transform,
    pub text: &'static mut Text,
    pub font: &'static mut TextFont,
    pub color: &'static mut TextColor,
    pub layout: &'static mut TextLayout,
    pub anchor: &'static mut Anchor,
    pub visibility: &'static mut Visibility,
}

pub fn render_text(mut commands: Commands, mut queue: ResMut<TextQueue>, mut query: Query<TextItem, With<ImmediateText>>) {
    let mut drawn_count = 0;

    // 1. MATCH EXISTING ENTITIES
    for (command, mut text_item) in queue.0.iter().zip(query.iter_mut()) {

        // Update content
        text_item.text.0 = command.text.clone(); // The string content

        // Update position
        text_item.transform.translation = Vec3::new(command.position.x, command.position.y, 1.0);

        // Update style
        text_item.font.font_size = command.size;
        text_item.color.0 = command.color;

        *text_item.visibility = Visibility::Visible;
        drawn_count += 1;
    }

    // 2. SPAWN NEW ENTITIES
    if queue.0.len() > drawn_count {
        for command in queue.0.iter().skip(drawn_count) {
            commands.spawn((
                Text2d::new(command.text.clone()),
                Transform::from_xyz(command.position.x, command.position.y, 1.0),
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
    for mut text_item in query.iter_mut().skip(drawn_count) {
        *text_item.visibility = Visibility::Hidden;
    }

    // 4. CLEAR
    queue.0.clear();
}