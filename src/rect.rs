use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

use crate::GameState;

pub struct RectPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for RectPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(drop_rect)
                .with_system(move_rect),
        );
    }
}

fn drop_rect(keyboard_input: Res<Input<KeyCode>>, mut commands: Commands) {
    if keyboard_input.just_pressed(KeyCode::R) {
        info!("add a rect");
        commands
            .spawn_bundle(twisted_rect())
            .insert(TwistedRect)
            .insert(Name::from("rect"));
    }
}

fn move_rect(mut rects: Query<(&mut Transform,), With<TwistedRect>>) {}

#[derive(Component)]
struct TwistedRect;

const TWISTED_RECT: &str = "M0 0L100 40V99L0 59V0Z";

fn twisted_rect() -> ShapeBundle {
    GeometryBuilder::build_as(
        &shapes::SvgPathShape {
            svg_path_string: TWISTED_RECT.to_owned(),
            svg_doc_size_in_px: Vec2::new(100., 100.),
        },
        DrawMode::Fill(FillMode::color(Color::AZURE)),
        Transform::default(),
    )
}
