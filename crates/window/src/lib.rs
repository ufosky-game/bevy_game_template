use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window, WindowPlugin};
use bevy::winit::WinitWindows;
use std::io::Cursor;
use winit::window::Icon;

pub fn configure_window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Bevy game".to_string(), // ToDo
            resolution: (800., 600.).into(),
            canvas: Some("#bevy".to_owned()),
            ..default()
        }),
        ..default()
    }
}

pub struct WindowIconPlugin;

impl Plugin for WindowIconPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(set_window_icon.in_schedule(CoreSchedule::Startup));
    }
}

// Sets the icon on windows and X11
fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_entity = primary_window.single();
    let primary = windows.get_window(primary_entity).unwrap();
    let icon_buf = Cursor::new(include_bytes!(
        "../build/macos/AppIcon.iconset/icon_256x256.png"
    ));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}
