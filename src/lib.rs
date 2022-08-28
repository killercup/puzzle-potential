#![allow(clippy::type_complexity)]

mod actions;
mod audio;
mod color;
mod draggable;
mod loading;
mod menu;
mod player;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;

#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::{app::App, render::camera::RenderTarget};
use bevy_mod_picking::PickingCameraBundle;
use bevy_prototype_lyon::prelude::ShapePlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading);

        app.add_startup_system(camera_setup);

        app.add_plugin(LoadingPlugin);
        app.add_plugin(MenuPlugin);
        // app.add_plugin(ActionsPlugin);
        app.add_plugin(InternalAudioPlugin);
        // app.add_plugin(PlayerPlugin);
        app.add_plugin(ShapePlugin);
        app.add_plugin(color::ColorCirclePlugin);
        app.add_plugin(draggable::DragPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}

fn camera_setup(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(MainCamera)
        .insert_bundle(PickingCameraBundle::default());
}

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;
