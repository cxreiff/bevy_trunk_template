use bevy::prelude::*;

mod config_plugin;
mod camera_plugin;
mod loading_plugin;
mod kitty_plugin;

use config_plugin::ConfigPlugin;
use loading_plugin::LoadingPlugin;
use camera_plugin::CameraPlugin;
use kitty_plugin::KittyPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    Loading,
    Playing,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ConfigPlugin)
            .add_plugin(LoadingPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(KittyPlugin);
    }
}
