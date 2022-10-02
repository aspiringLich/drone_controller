use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{FillMode, *};
use bevy_rapier2d::prelude::*;
use lazy_static::lazy_static;
use std::{
    convert::identity,
    f32::consts::{PI, TAU},
};

use crate::AIR_RESISTANCE;

lazy_static! {
    static ref ROUNDED_STROKE_OPTIONS: StrokeOptions = {
        let mut out: StrokeOptions = default();
        out.start_cap = LineCap::Round;
        out.end_cap = LineCap::Round;
        out.line_join = LineJoin::Round;
        return out;
    };
}

fn rounded_stroke_mode(line_width: f32, line_color: Color) -> StrokeMode {
    StrokeMode {
        options: ROUNDED_STROKE_OPTIONS.with_line_width(line_width),
        color: line_color,
    }
}

pub fn rounded_draw_mode(line_width: f32, line_color: Color, fill_color: Color) -> DrawMode {
    DrawMode::Outlined {
        outline_mode: rounded_stroke_mode(line_width, line_color),
        fill_mode: FillMode {
            options: FillOptions::default(),
            color: fill_color,
        },
    }
}
// * CONSTANTS FOR DIMENSIONS OF DRONE *//
const D_SIZE: f32 = 32.0; // the overall size of the drone
const SHELL_SIZE: f32 = 1.1; // how big the shell of the drone is in comparison to the rest of it
const SHELL_ANGLE: f32 = TAU * 0.6; // arc the shell occupies
const EYE_SIZE: f32 = 0.45; // size of the "eye"
const LINE_WIDTH: f32 = 2.0; // self-explanatory
const FRAME_WIDTH: f32 = LINE_WIDTH * 2.5; // "Frame" width, used for the basis of the "frame" attatching the main body to the thrusters
const THRUSTER_DISTANCE: f32 = D_SIZE * 3.;
const FRAME_HEIGHT: f32 = D_SIZE * 0.8;
const THRUSTER_RADIUS: f32 = D_SIZE / 3.0;
const NOZZLE_LEN: f32 = D_SIZE * 0.3;

pub fn spawn_drone(commands: &mut Commands) {
    let shell_color = Color::rgb_u8(99, 155, 255);
    let shell_draw_mode = rounded_draw_mode(LINE_WIDTH, shell_color, shell_color);

    let inner_circle = shapes::Circle {
        radius: D_SIZE,
        center: default(),
    };

    // the angle the point should start at to draw the arc upwards
    const START_ANGLE: f32 = (PI - SHELL_ANGLE) / 2.0;
    let start_pos = Vec2::new(f32::cos(START_ANGLE), f32::sin(START_ANGLE));

    let outer_shell = {
        let mut outer_shell = PathBuilder::new();
        // outer_shell.move_to(start_pos * D_SIZE * EYE_SIZE);
        // outer_shell.arc(Vec2::ZERO, Vec2::ONE * D_SIZE * EYE_SIZE, SHELL_ANGLE, 1.0);
        outer_shell.line_to(start_pos * D_SIZE * SHELL_SIZE);
        outer_shell.arc(
            Vec2::ZERO,
            Vec2::ONE * D_SIZE * SHELL_SIZE,
            SHELL_ANGLE,
            1.0,
        );
        outer_shell.line_to(Vec2::ZERO);
        outer_shell.line_to(start_pos * D_SIZE * SHELL_SIZE);
        outer_shell.build()
    };
    let main_body = shapes::Circle {
        radius: D_SIZE,
        center: Vec2::ZERO,
    };
    let eye = shapes::Circle {
        radius: D_SIZE * EYE_SIZE,
        center: Vec2::ZERO,
    };
    let frame_stroke_mode = StrokeMode {
        options: ROUNDED_STROKE_OPTIONS.with_line_width(FRAME_WIDTH),
        color: Color::rgb_u8(80, 80, 80),
    };

    let frame = {
        let mut frame = PathBuilder::new();
        frame.move_to(Vec2::new(-THRUSTER_DISTANCE, 0.0));
        frame.line_to(Vec2::new(0.0, -FRAME_HEIGHT * 0.25));
        frame.line_to(Vec2::new(THRUSTER_DISTANCE, 0.0));
        frame.line_to(Vec2::new(0.0, FRAME_HEIGHT));
        frame.line_to(Vec2::new(-THRUSTER_DISTANCE, 0.0));
        frame.build()
    };
    let frame_pad_stroke_mode = StrokeMode {
        options: ROUNDED_STROKE_OPTIONS.with_line_width(FRAME_WIDTH * 2.0),
        color: Color::rgb_u8(71, 124, 217),
    };
    let frame_pad = {
        const PCT: f32 = 0.6;
        const PCB: f32 = 0.5;
        let mut frame_pad = PathBuilder::new();
        frame_pad.move_to(Vec2::new(
            -THRUSTER_DISTANCE * PCT,
            FRAME_HEIGHT * (1.0 - PCT),
        ));
        frame_pad.line_to(Vec2::new(0.0, FRAME_HEIGHT));
        frame_pad.line_to(Vec2::new(
            THRUSTER_DISTANCE * PCT,
            FRAME_HEIGHT * (1.0 - PCT),
        ));
        frame_pad.move_to(Vec2::new(
            -THRUSTER_DISTANCE * PCB,
            -FRAME_HEIGHT * 0.25 * PCB,
        ));
        frame_pad.line_to(Vec2::new(0.0, -FRAME_HEIGHT * 0.25));
        frame_pad.line_to(Vec2::new(
            THRUSTER_DISTANCE * PCB,
            -FRAME_HEIGHT * 0.25 * PCB,
        ));

        frame_pad.build()
    };
    let thruster_ball = shapes::Circle {
        radius: THRUSTER_RADIUS,
        ..default()
    };
    let thruster_nozzle = shapes::Rectangle {
        extents: Vec2::new(NOZZLE_LEN * 1.3, NOZZLE_LEN),
        origin: RectangleOrigin::Center,
    };

    fn spawn_geometry<'a>(
        commands: &mut Commands,
        shape: &impl Geometry,
        draw_mode: DrawMode,
        translation: Vec3,
        name: &'a str,
    ) -> Entity {
        let name = name.to_string();
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                shape,
                draw_mode,
                Transform {
                    translation,
                    ..default()
                },
            ))
            .insert(Name::new(name))
            .id()
    };

    let mut ids: Vec<Entity> = vec![];

    ids.push(spawn_geometry(
        commands,
        &outer_shell,
        shell_draw_mode,
        Vec3::new(0., 0., 0.8),
        "drone_outer_shell.geo",
    ));

    ids.push(spawn_geometry(
        commands,
        &main_body,
        DrawMode::Fill(FillMode::color(Color::GRAY)),
        Vec3::new(0., 0., 0.7),
        "drone_main_body.geo",
    ));

    ids.push(spawn_geometry(
        commands,
        &eye,
        DrawMode::Fill(FillMode::color(Color::GREEN)),
        Vec3::new(0., 0., 0.9),
        "drone_eye.geo",
    ));

    ids.push(spawn_geometry(
        commands,
        &frame,
        DrawMode::Stroke(frame_stroke_mode),
        Vec3::new(0., 0., 0.5),
        "drone_frame.geo",
    ));

    ids.push(spawn_geometry(
        commands,
        &frame_pad,
        DrawMode::Stroke(frame_pad_stroke_mode),
        Vec3::new(0., 0., 0.6),
        "drone_frame_pad.geo",
    ));

    let mut spawn_thruster = |index: usize| {
        let translation = [
            Vec3::new(-THRUSTER_DISTANCE, 0.0, 0.8),
            Vec3::new(THRUSTER_DISTANCE, 0.0, 0.8),
        ];
        let iden_char = ["l", "r"];

        let thrusterid = commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: translation[index],
                    ..default()
                },
                sprite: Sprite {
                    custom_size: Some(Vec2::ZERO),
                    ..default()
                },
                ..default()
            })
            .insert(Name::new(format!(
                "drone_thruster_{}.parent",
                iden_char[index]
            )))
            .id();

        let id = spawn_geometry(
            commands,
            &thruster_ball,
            shell_draw_mode,
            Vec3::ZERO,
            &format!("drone_thruster_{}_ball.geo", iden_char[index]),
        );
        commands.entity(thrusterid).add_child(id);

        let id = spawn_geometry(
            commands,
            &thruster_nozzle,
            DrawMode::Fill(FillMode::color(Color::GRAY)),
            Vec3::new(0., -THRUSTER_RADIUS, -0.1),
            &format!("drone_thruster_{}_nozzle.geo", iden_char[index]),
        );
        commands.entity(thrusterid).add_child(id);

        return thrusterid;
    };

    ids.push(spawn_thruster(0));
    ids.push(spawn_thruster(1));

    let droneid = commands
        .spawn()
        .insert(Name::new("drone.parent"))
        .insert_bundle(SpriteBundle::default())
        .push_children(ids.as_slice())
        .id();

    // * PHYSICS SHMISICS * //
    commands
        .entity(droneid)
        .insert(RigidBody::Dynamic)
        .insert(Collider::compound(vec![
            (Vec2::ZERO, 0.0, Collider::ball(D_SIZE)),
            (
                Vec2::X * THRUSTER_DISTANCE,
                0.0,
                Collider::ball(THRUSTER_RADIUS),
            ),
            (
                Vec2::X * -THRUSTER_DISTANCE,
                0.0,
                Collider::ball(THRUSTER_RADIUS),
            ),
        ]))
        .insert(Velocity::zero())
        .insert(AIR_RESISTANCE);
}
