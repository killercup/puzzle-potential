use bevy::prelude::*;

use bevy::diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy_editor_pls::prelude::*;
use bevy_mod_picking::DebugEventsPickingPlugin;

pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin);
        app.add_plugin(EntityCountDiagnosticsPlugin);
        app.add_plugin(EditorPlugin);
        app.add_plugin(DebugEventsPickingPlugin);
    }
}
