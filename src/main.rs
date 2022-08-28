// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::{App, ClearColor, Color, Msaa, NonSend, WindowDescriptor};
use bevy::window::WindowId;
use bevy::winit::WinitWindows;
use bevy::DefaultPlugins;
use bevy_game::GamePlugin;
use bevy_mod_picking::{DebugEventsPickingPlugin, DefaultPickingPlugins};
use std::io::Cursor;
use winit::window::Icon;

#[cfg(feature = "dev")]
use bevy_editor_pls::prelude::*;

fn main() {
    let mut app = App::new();
    app.insert_resource(Msaa { samples: 1 });
    app.insert_resource(ClearColor(Color::hsl(1.0, 1.0, 0.99)));
    app.insert_resource(WindowDescriptor {
        width: 1200.,
        height: 800.,
        title: "Cheesy color combine challenge".to_string(),
        canvas: Some("#bevy".to_owned()),
        ..Default::default()
    });
    app.add_plugins(DefaultPlugins);
    app.add_plugins(DefaultPickingPlugins);

    app.add_plugin(GamePlugin);
    app.add_startup_system(set_window_icon);

    #[cfg(feature = "dev")]
    app.add_plugin(EditorPlugin);
    #[cfg(feature = "dev")]
    app.add_plugin(DebugEventsPickingPlugin);

    app.run();
}

// Sets the icon on windows and X11
fn set_window_icon(windows: NonSend<WinitWindows>) {
    let primary = windows.get_window(WindowId::primary()).unwrap();
    let icon_buf = Cursor::new(include_bytes!("../assets/textures/bevy.png"));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}
