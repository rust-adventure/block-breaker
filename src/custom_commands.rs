use bevy::{
    ecs::system::Command, prelude::*,
    sprite::MaterialMesh2dBundle,
};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    board::{self, Board},
    levels::LEVEL_1,
    Ball, Damage, Powerup,
};

pub struct SpawnBall {
    pub velocity: Velocity,
    pub transform: Transform,
}

impl Command for SpawnBall {
    fn write(self, world: &mut World) {
        let shape = shapes::Circle {
            radius: 10.0,
            ..Default::default()
        };

        world
            .spawn()
            .insert_bundle(GeometryBuilder::build_as(
                &shape,
                DrawMode::Outlined {
                    fill_mode: bevy_prototype_lyon::prelude::FillMode::color(
                        Color::WHITE,
                    ),
                    outline_mode: StrokeMode::new(
                        Color::BLACK,
                        1.0,
                    ),
                },
                self.transform,
            ))
            // material mesh bundle is only applicable in
            // bevy
            // 0.8.0 .spawn_bundle(MaterialMesh2dBundle
            // {     mesh: meshes
            //         .add(
            //
            // bevy::prelude::shape::Circle::new(50.)
            //                 .into(),
            //         )
            //         .into(),
            //     material: materials
            //         .add(ColorMaterial::from(Color::
            // PURPLE)),     transform:
            // Transform::from_xyz(
            //         board.physical.x / 2.0,
            //         50.0,
            //         0.0,
            //     ),
            //     ..default()
            // })
            .insert(RigidBody::Dynamic)
            .insert(Restitution {
                coefficient: 1.0,
                combine_rule: CoefficientCombineRule::Min,
            })
            .insert(Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            })
            // .insert(
            //     ColliderMassProperties::Density,
            // )
            .insert(Collider::ball(10.0))
            .insert(self.velocity)
            .insert(Ball)
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(GravityScale(0.0));
    }
}

pub struct SpawnPowerup {
    pub transform: Transform,
}

impl Command for SpawnPowerup {
    fn write(self, world: &mut World) {
        let capsule = {
            let mut meshes = world
                .get_resource_mut::<Assets<Mesh>>()
                .unwrap();
            meshes
                .add(
                    shape::Capsule {
                        radius: 10.0,
                        // rings: todo!(),
                        depth: 40.0,
                        // latitudes: todo!(),
                        // longitudes: todo!(),
                        // uv_profile: todo!(),
                        ..Default::default()
                    }
                    .into(),
                )
                .into()
        };
        let color_material = {
            let mut materials = world
                .get_resource_mut::<Assets<ColorMaterial>>()
                .unwrap();
            materials
                .add(ColorMaterial::from(Color::PURPLE))
        };

        world
            .spawn()
            .insert_bundle(MaterialMesh2dBundle {
                mesh: capsule,
                material: color_material,
                transform: self
                    .transform
                    .clone()
                    .with_rotation(Quat::from_rotation_z(
                        std::f32::consts::FRAC_PI_2,
                    )),
                ..default()
            })
            .insert(Sensor)
            .insert(Restitution {
                coefficient: 1.0,
                combine_rule: CoefficientCombineRule::Min,
            })
            .insert(Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            })
            // .insert(
            //     ColliderMassProperties::Density,
            // )
            .insert(Collider::capsule(
                Vec2::new(-20.0, 20.0),
                Vec2::new(-20.0, 20.0),
                10.0,
            ))
            .insert(Velocity::linear(Vec2::new(
                0.0, -400.0,
            )))
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(Powerup::TripleBall);
    }
}

///
///
///

pub struct SpawnLevel {
    pub level: usize,
}

impl Command for SpawnLevel {
    fn write(self, world: &mut World) {
        let board =
            world.get_resource::<Board>().unwrap().clone();
        for (row_index, row) in LEVEL_1.iter().enumerate() {
            for (column_index, column) in
                row.iter().enumerate()
            {
                if let Some(block) = column {
                    println!(
                        "brick {} {}",
                        row_index, column_index
                    );
                    world.spawn()
                        .insert_bundle(SpriteBundle {
                            sprite: Sprite {
                                color: block.color(),
                                // color: Color::rgba(
                                //     0.0, 0.0, 0.0, 0.0,
                                // ),
                                custom_size: Some(Vec2::new(
                                    board::TILE_X_SIZE,
                                    board::TILE_Y_SIZE,
                                )),
                                // anchor: Anchor::BottomLeft,
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(
                                board.physical.x / 2.0
                                    + board
                                        .u8_cell_to_physical(
                                            11 - column_index
                                                as u8,
                                            board::Axis::X,
                                        )
                                    - board::TILE_X_SIZE,
                                board.physical.y / 2.0
                                    + board
                                        .u8_cell_to_physical(
                                            28 - row_index
                                                as u8,
                                            board::Axis::Y,
                                        ),
                                4.0,
                            ),
                            ..Default::default()
                        })
                        .insert(RigidBody::Fixed)
                        .insert(Collider::cuboid(board::TILE_X_SIZE / 2.0, board::TILE_X_SIZE / 2.0))
                        .insert(Restitution {
                            coefficient: 1.0,
                            combine_rule: CoefficientCombineRule::Min,
                        })
                        .insert(Friction {
                            coefficient: 0.0,
                            combine_rule: CoefficientCombineRule::Min,
                        })
                        // .insert(
                        //     ColliderMassProperties::Density,
                        // )
                        .insert(*block)
                        .insert(Damage(0));
                }
            }
        }
    }
}
