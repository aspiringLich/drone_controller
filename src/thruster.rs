use bevy_prototype_debug_lines::*;
use std::f32::consts::PI;
use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::setup::*;

#[derive(Default)]
pub struct ThrusterInfo {
    rotation: [f32; 2],
    power: [f32; 2],
}

pub fn apply_thruster_force(
    mut commands: Commands,
    thruster_info: Res<ThrusterInfo>,
    entities: Res<DroneEntities>,
    config: Res<RapierConfiguration>,
    mut q_transform: Query<&mut Transform>,
    mut lines: ResMut<DebugLines>,
) {
    let gravity = config.gravity;

    let drone = entities.drone;
    let transform = q_transform.get(drone).unwrap();
    let rotation = transform.rotation.mul_vec3(Vec3::X).truncate();
    let translation = transform.translation;

    let lthruster = entities.lthruster;
    let l_transform = q_transform.get(lthruster).unwrap();
    let l_rotation = l_transform
        .rotation
        .mul_vec3(Vec3::X)
        .truncate()
        .rotate(rotation);
    let v = l_transform.translation.truncate().rotate(l_rotation);
    let l_translation = Vec3::new(v.x, v.y, l_transform.translation.z) + translation;

    // let rthruster = entities.rthruster;
    // let r_rotation = q_transform.get(rthruster).unwrap().rotation.z + rotation - PI / 2.0;

    let l_force = (l_rotation * FORCE / 10.0).rotate(Vec2::Y);
    lines.line(
        l_translation,
        l_translation + Vec3::new(l_force.x, l_force.y, 0.0),
        0.0,
    );

    commands.entity(lthruster).insert(ExternalForce {
        force: l_force,
        torque: 0.0,
    });

    // commands.entity(rthruster).insert(ExternalForce {
    //     force: Vec2::Y * 500000.0 * Vec2::new(f32::cos(r_rotation), f32::sin(r_rotation)),
    //     torque: 0.0,
    // });
}
