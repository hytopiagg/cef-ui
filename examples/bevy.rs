use bevy::{app::App, prelude::Update, window::close_on_esc, DefaultPlugins};
use cef_ui::bindings::cef_settings_t;

fn main() {
    let settings = cef_settings_t {};

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, close_on_esc)
        .run();
}
