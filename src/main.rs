use autodefault::autodefault;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use lazy_static::lazy_static;

mod setup;
use setup::*;

pub const PIXELS_PER_METER: f32 = 32.0;
pub const AIR_RESISTANCE: Damping = Damping {
    linear_damping: 1.003,
    angular_damping: 1.003,
};

fn main() {
    App::new()
        // resources
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, -1024.),
            ..default()
        })
        // default setup plugins
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(ShapePlugin)
        .add_startup_system(spawn)
        .run();
}

fn spawn(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
    spawn_drone(&mut commands);
}
