// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::{App, ClearColor, Color, Msaa, NonSend, WindowDescriptor};
use bevy::window::{PresentMode, WindowId};
use bevy::winit::WinitWindows;
use bevy::DefaultPlugins;
use laser_beam::GamePlugin;
use std::io::Cursor;
use winit::window::Icon;

pub const TITLE: &str = "LASER BEAM!";

#[cfg(target_os = "wasm32")]
fn window_size() -> (f32, f32) {
    (948., 533.)
}

#[cfg(not(target_os = "wasm32"))]
fn window_size() -> (f32, f32) {
    (1280., 720.)
}

fn main() {
    let size = window_size();

    App::new()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(WindowDescriptor {
            width: size.0,
            height: size.1,
            title: TITLE.to_string(),
            canvas: Some("#bevy".to_owned()),
            present_mode: PresentMode::Immediate,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_startup_system(set_window_icon)
        .run();
}

// Sets the icon on windows and X11
fn set_window_icon(windows: NonSend<WinitWindows>) {
    let primary = windows.get_window(WindowId::primary()).unwrap();
    let icon_buf = Cursor::new(include_bytes!("../assets/textures/app_icon.png"));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}
