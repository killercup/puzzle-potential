use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::{app::App, render::camera::RenderTarget};
use bevy_mod_picking::Hover;

/// Tag components with this to make them draggable
#[derive(Component)]
pub struct Draggable;

#[derive(Default)]
struct Dragging {
    entity: Option<Entity>,
}

struct Dropped {
    entity: Entity,
    transform: Transform,
}

pub struct DragPlugin;

/// This plugin moves components on drag/drop
impl Plugin for DragPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Dragging::default());
        app.add_event::<Dropped>();

        app.add_system(drag_start);
        app.add_system(drag);
        app.add_system(dropped);
    }
}

fn drag_start(
    mouse_button: Res<Input<MouseButton>>,
    draggables: Query<(&Hover, Entity), With<Draggable>>,
    mut dragging: ResMut<Dragging>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }
    if let Some((_, entity)) = draggables.iter().find(|(hover, ..)| hover.hovered()) {
        *dragging = Dragging {
            entity: Some(entity),
        };
    }
}

fn drag(
    mouse_button: Res<Input<MouseButton>>,
    cursor_pos: CursorToWorldPosition,
    dragging: Res<Dragging>,
    mut draggables: Query<(Entity, &mut Transform), With<Draggable>>,
) {
    if !mouse_button.pressed(MouseButton::Left) {
        return;
    }

    let entity = if let Some(entity) = dragging.entity {
        entity
    } else {
        return;
    };

    let mut transform =
        if let Some((_, transform)) = draggables.iter_mut().find(|(e, ..)| *e == entity) {
            transform
        } else {
            return;
        };

    let cursor = if let Some(pos) = cursor_pos.world_position() {
        pos
    } else {
        return;
    };

    *transform = transform.with_translation(cursor.extend(0.0));
}

fn dropped(
    mouse_button: Res<Input<MouseButton>>,
    mut dropped: EventWriter<Dropped>,
    mut dragging: ResMut<Dragging>,
) {
    if !mouse_button.just_released(MouseButton::Left) {
        return;
    }

    dragging.entity = None;
}

// from https://github.com/bevy-cheatbook/bevy-cheatbook/blob/5ed41b21f8dadeed9aceea985faa636501b7a1f4/src/code/examples/cursor2world.rs
#[derive(SystemParam)]
struct CursorToWorldPosition<'w, 's> {
    windows: Res<'w, Windows>,
    camera: Query<'w, 's, (&'static Camera, &'static GlobalTransform), With<crate::MainCamera>>,
}

impl<'w, 's> CursorToWorldPosition<'w, 's> {
    fn world_position(&self) -> Option<Vec2> {
        let (camera, camera_transform) = self.camera.get_single().ok()?;
        let window = if let RenderTarget::Window(id) = camera.target {
            self.windows.get(id)?
        } else {
            self.windows.get_primary()?
        };

        if let Some(screen_pos) = window.cursor_position() {
            let window_size = Vec2::new(window.width() as f32, window.height() as f32);
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
            let ndc_to_world =
                camera_transform.compute_matrix() * camera.projection_matrix().inverse();
            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
            Some(world_pos.truncate())
        } else {
            None
        }
    }
}
