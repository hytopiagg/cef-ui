use bevy::{app::App, prelude::Update, window::close_on_esc, DefaultPlugins};
use wrapper::MainArgs;

fn main() {
    let _main_args = MainArgs::new(vec!["foobar".to_string()]);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, close_on_esc)
        .run();
}
