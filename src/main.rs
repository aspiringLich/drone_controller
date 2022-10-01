use autodefault::autodefault;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use lazy_static::lazy_static;

mod setup;
use setup::*;

fn main() {
    App::new()
        // default setup plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(spawn)
        .run();
}

fn spawn(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
    spawn_drone(&mut commands);
}
