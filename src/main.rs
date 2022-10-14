use std::env;

use autodefault::autodefault;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_prototype_debug_lines::DebugLinesPlugin;
use bevy_prototype_lyon::prelude::ShapePlugin;
use bevy_rapier2d::prelude::*;
use lazy_static::lazy_static;

mod setup;
use setup::*;

mod thruster;
use thruster::*;

pub const PHYS_SCALE: f32 = 256.0;
pub const AIR_RESISTANCE: Damping = Damping {
    linear_damping: 1.003,
    angular_damping: 0.0,
};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    App::new()
        // resources
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, -1.0),
            ..default()
        })
        .init_resource::<ThrusterInfo>()
        .init_resource::<DroneEntities>()
        // default setup plugins
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default().with_physics_scale(PHYS_SCALE))
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(ShapePlugin)
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(spawn)
        // systems
        .add_system(apply_thruster_force)
        .run();
}

fn spawn(mut commands: Commands, mut entities: ResMut<DroneEntities>) {
    commands.spawn_bundle(Camera2dBundle::default());
    spawn_drone(&mut commands, entities);
}
