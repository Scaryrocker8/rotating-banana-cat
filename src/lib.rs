use bevy::prelude::*;
use loading::LoadingPlugin;

mod loading;

// Plugins

// Game Plugin
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app // Bevy default plugins
            .add_plugins(LoadingPlugin); // Asset loading plugin
    }
}
