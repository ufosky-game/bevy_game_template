// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins(DefaultPlugins.set(window::configure_window_plugin()))
        .add_plugin(game::GamePlugin)
        .add_plugin(asset_loading::LoadingPlugin)
        .add_plugin(window::WindowIconPlugin)
        .run();
}
