use block_breaker::{
    assets::{AssetsPlugin, ImageAssets},
    blocks::{block_removal, Block},
    board::*,
    custom_commands::*,
    scoring::ScorePlugin,
    ui::UiPlugin,
    SpawnThreeBallsEvent, *,
};

use bevy::{
    prelude::*,
    render::{
        render_resource::WgpuFeatures,
        settings::WgpuSettings,
    },
    sprite::{Anchor, MaterialMesh2dBundle},
};
use bevy_hanabi::*;
use bevy_prototype_lyon::prelude::*;
use heron::{
    prelude::*,
    rapier_plugin::{
        nalgebra::Point2,
        rapier2d::prelude::ColliderBuilder,
    },
    CustomCollisionShape,
};
use iyes_loopless::prelude::*;

fn main() {
    let mut options = WgpuSettings::default();
    options.features.set(
        WgpuFeatures::VERTEX_WRITABLE_STORAGE,
        true,
    );
    App::new()
        .insert_resource(options)
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(Board::new(11, 28))
        .insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::WARN,
            filter: "bevy_hanabi=error,spawn=trace"
                .to_string(),
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(HanabiPlugin)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(UiPlugin)
        .add_plugin(AssetsPlugin)
        .insert_resource(ClearColor(Color::rgb(
            0.5, 0.5, 0.5,
        )))
        .insert_resource(Gravity::from(Vec3::new(
            0.0, 0.0, 0.0,
        )))
        .add_loopless_state(STARTING_GAME_STATE)
        .add_plugin(ScorePlugin)
        .add_event::<SpawnThreeBallsEvent>()
        .add_startup_system(setup)
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Playing)
                .with_system(paddle_collisions)
                .with_system(despawn_area_collisions)
                .with_system(ball_collisions)
                .with_system(movement)
                .with_system(track_damage)
                .with_system(block_removal)
                .with_system(powerup_gravity)
                .with_system(powerup_collisions)
                .with_system(three_balls_events)
                .into(),
        )
        .add_enter_system(
            GameState::Playing,
            spawn_new_game,
        )
        .run();
}

fn setup(
    mut commands: Commands,
    images: Res<ImageAssets>,
    board: Res<Board>,
    mut effects: ResMut<Assets<EffectAsset>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform::from_xyz(
        board.physical.x / 2.0,
        board.physical.y / 2.0,
        1000.0,
    );
    camera.orthographic_projection.scale = 2.0;
    commands.spawn_bundle(camera);

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            // color: todo!(),
            custom_size: Some(Vec2::new(
                1920.0 * 2.0,
                1080.0 * 2.0,
            )),
            anchor: Anchor::Center,
            ..Default::default()
        },
        transform: Transform::from_xyz(
            board.physical.x / 2.0,
            board.physical.y / 2.0,
            0.0,
        ),
        texture: images.background.clone(),
        ..Default::default()
    });
    // setup effects
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1.0, 1.0, 1.0, 1.0));
    gradient.add_key(0.05, Vec4::new(1.0, 1.0, 1.0, 0.0));

    let mut size_gradient = Gradient::new();
    size_gradient.add_key(0.0, Vec2::new(10.0, 10.0));
    size_gradient.add_key(0.05, Vec2::new(100.0, 100.0));

    let spawner = Spawner::once(1.0.into(), false);
    let effect = effects.add(
        EffectAsset {
            name: "Impact".into(),
            capacity: 32768,
            spawner,
            ..Default::default()
        }
        // .init(PositionCircleModifier {
        //     radius: 10.05,
        //     speed: 1.2.into(),
        //     dimension: ShapeDimension::Surface,
        //     ..Default::default()
        // })
        .render(ParticleTextureModifier {
            texture: images.ball_hit.clone(),
        })
        .render(SizeOverLifetimeModifier {
            gradient:size_gradient
            //  Gradient::constant(Vec2::splat(
            //     100.05,
            // )),
        })
        .render(ColorOverLifetimeModifier { gradient }),
    );
    commands
        .spawn_bundle(
            ParticleEffectBundle::new(effect)
                .with_spawner(spawner),
        )
        .insert(Name::new("effect"))
        .insert(BallContactEffect);
}

fn spawn_new_game(
    mut commands: Commands,
    images: Res<ImageAssets>,
    board: Res<Board>,
) {
    let shape = shapes::Circle {
        radius: 10.0,
        ..Default::default()
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::WHITE),
                outline_mode: StrokeMode::new(
                    Color::BLACK,
                    1.0,
                ),
            },
            Transform::from_xyz(
                board.physical.x / 2.0,
                board.physical.y / 2.0
                    + board.u8_cell_to_physical(
                        3,
                        board::Axis::Y,
                    )
                    + 100.0,
                5.0,
            ),
        ))
        // material mesh bundle is only applicable in bevy
        // 0.8.0 .spawn_bundle(MaterialMesh2dBundle
        // {     mesh: meshes
        //         .add(
        //
        // bevy::prelude::shape::Circle::new(50.)
        //                 .into(),
        //         )
        //         .into(),
        //     material: materials
        //         .add(ColorMaterial::from(Color::PURPLE)),
        //     transform: Transform::from_xyz(
        //         board.physical.x / 2.0,
        //         50.0,
        //         0.0,
        //     ),
        //     ..default()
        // })
        .insert(RigidBody::Dynamic)
        .insert(PhysicMaterial {
            restitution: 1.0,
            friction: 0.0,
            ..Default::default()
        })
        .insert(CollisionShape::Sphere { radius: 10.0 })
        .insert(Velocity::from_linear(Vec3::new(
            100.0, 400.0, 0.0,
        )))
        .insert(Ball)
        .insert(RotationConstraints::lock());

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(200.0, 20.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(
                board.physical.x / 2.0,
                board.physical.y / 2.0
                    + board.u8_cell_to_physical(
                        3,
                        board::Axis::Y,
                    ),
                5.0,
            ),
            ..Default::default()
        })
        .insert(RigidBody::KinematicVelocityBased)
        .insert(PhysicMaterial {
            restitution: 1.0,
            friction: 0.0,
            ..Default::default()
        })
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(100.0, 10.0, 10.0),
            border_radius: None,
        })
        .insert(Velocity::from_linear(Vec3::new(
            0.0, 0.0, 0.0,
        )))
        .insert(Paddle);

    // Playing Area Exterior

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::Rgba {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
                alpha: 0.3,
            },
            custom_size: Some(Vec2::new(
                board.physical.x,
                board.physical.y,
            )),
            anchor: Anchor::BottomLeft,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
        ..Default::default()
    });

    // border
    let shape = shapes::Rectangle {
        extents: Vec2::new(
            board.physical.x + 10.0,
            board.physical.y + 10.0,
        ),
        ..Default::default()
    };

    commands.spawn_bundle(GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::rgba(
                0.0, 0.0, 0.0, 0.0,
            )),
            outline_mode: StrokeMode::new(
                Color::rgba(82.0, 90.0, 94.0, 1.0),
                10.0,
            ),
        },
        Transform::from_xyz(
            board.physical.x / 2.0,
            board.physical.y / 2.0,
            2.0,
        ),
    ));

    commands
        .spawn()
        .insert(GlobalTransform::default())
        .insert(RigidBody::Static)
        .insert(PhysicMaterial {
            restitution: 1.0,
            friction: 0.0,
            ..Default::default()
        })
        .insert(CollisionShape::Custom {
            shape: CustomCollisionShape::new(
                ColliderBuilder::polyline(
                    vec![
                        Point2::new(0.0, 0.0),
                        Point2::new(board.physical.x, 0.0),
                        Point2::new(
                            board.physical.x,
                            board.physical.y,
                        ),
                        Point2::new(0.0, board.physical.y),
                    ],
                    // None
                    Some(vec![
                        [0, 1],
                        [1, 2],
                        [2, 3],
                        [3, 0],
                    ]),
                ),
            ),
        })
        .insert(PlayingAreaBorder);

    // death area
    commands
        .spawn()
        .insert(GlobalTransform::default())
        .insert(Transform::from_xyz(
            board.physical.x / 2.0,
            (board.physical.y / 2.0
                + board.u8_cell_to_physical(
                    3,
                    board::Axis::Y,
                ))
                / 2.0,
            0.0,
        ))
        .insert(RigidBody::Sensor)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(
                board.physical.x / 2.0,
                (board.physical.y / 2.0
                    + board.u8_cell_to_physical(
                        3,
                        board::Axis::Y,
                    ))
                    / 2.0,
                0.0,
            ),
            border_radius: None,
        })
        .insert(DespawnArea);

    commands.add(SpawnLevel { level: 1 });
}
fn paddle_collisions(
    mut events: EventReader<CollisionEvent>,
    paddles: Query<(Entity, &Transform), With<Paddle>>,
    mut ball: Query<
        (&mut Velocity, &Transform),
        With<Ball>,
    >,
) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(a, b) => {
                let colliders = if let (Ok(a), Ok(b)) = (
                    ball.get_mut(a.rigid_body_entity()),
                    paddles.get(b.rigid_body_entity()),
                ) {
                    Some((a, b))
                } else if let (Ok(a), Ok(b)) = (
                    ball.get_mut(b.rigid_body_entity()),
                    paddles.get(a.rigid_body_entity()),
                ) {
                    Some((a, b))
                } else {
                    None
                };

                if let Some((
                    (mut velocity, ball_transform),
                    (_, paddle_transform),
                )) = colliders
                {
                    let x_diff = ball_transform
                        .translation
                        .x
                        - paddle_transform.translation.x;

                    // a^2 + b^2 = c^2
                    let optimal_velocity: f32 =
                        100.0 * 100.0 + 400.0 * 400.0;
                    let c = optimal_velocity.sqrt();

                    // TODO: Jacob says this `10` might need
                    // to be a function of the paddle width
                    let normalized = Vec2::new(
                        x_diff * 7.5,
                        velocity.linear.y,
                    )
                    .normalize();

                    // expand normalized parts back out into
                    // full magnitude
                    let new_velocity = normalized * c;

                    *velocity =
                        Velocity::from_linear(Vec3::new(
                            new_velocity.x,
                            new_velocity.y,
                            0.0,
                        ))
                }
            }
            CollisionEvent::Stopped(_, _) => {}
        }
    }
}

fn despawn_area_collisions(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    despawn_area: Query<Entity, With<DespawnArea>>,
    mut ball: Query<Entity, With<Ball>>,
) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(a, b) => {
                if let (Ok(entity), Ok(wall)) = (
                    ball.get_mut(a.rigid_body_entity()),
                    despawn_area.get(b.rigid_body_entity()),
                ) {
                    commands
                        .entity(entity)
                        .despawn_recursive();
                } else if let (Ok(entity), Ok(wall)) = (
                    ball.get_mut(b.rigid_body_entity()),
                    despawn_area.get(a.rigid_body_entity()),
                ) {
                    commands
                        .entity(entity)
                        .despawn_recursive();
                }
            }
            CollisionEvent::Stopped(_, _) => {
                // dbg!("stopped");
            }
        }
    }
}
fn ball_collisions(
    mut events: EventReader<CollisionEvent>,
    mut effect: Query<
        (&mut ParticleEffect, &mut Transform),
        (With<BallContactEffect>, Without<Ball>),
    >,

    ball: Query<(&Velocity, &Transform), With<Ball>>,
    board: Res<Board>,
) {
    let (mut effect, mut effect_transform) =
        effect.single_mut();

    for event in events.iter() {
        match event {
            CollisionEvent::Started(a, b) => {
                let collider = if let Ok(a) =
                    ball.get(a.rigid_body_entity())
                {
                    Some(a)
                } else if let Ok(b) =
                    ball.get(b.rigid_body_entity())
                {
                    Some(b)
                } else {
                    None
                };

                if let Some((velocity, ball_transform)) =
                    collider
                {
                    dbg!("ball hit");
                    // This isn't the most accurate place to spawn the particle effect,
                    // but this is just for demonstration, so whatever.
                    effect_transform.translation =
                        ball_transform.translation.clone();
                    // *effect_transform = Transform::from_xyz(
                    //     board.physical.x / 2.0,
                    //     board.physical.y / 2.0,
                    //     5.0,
                    // );
                    // Spawn the particles
                    effect.maybe_spawner().unwrap().reset();
                    // let x_diff = ball_transform
                    //     .translation
                    //     .x
                    //     - paddle_transform.translation.x;

                    // // a^2 + b^2 = c^2
                    // let optimal_velocity: f32 =
                    //     100.0 * 100.0 + 400.0 * 400.0;
                    // let c = optimal_velocity.sqrt();

                    // // TODO: Jacob says this `10` might need
                    // // to be a function of the paddle width
                    // let normalized = Vec2::new(
                    //     x_diff * 7.5,
                    //     velocity.linear.y,
                    // )
                    // .normalize();

                    // // expand normalized parts back out into
                    // // full magnitude
                    // let new_velocity = normalized * c;

                    // *velocity =
                    //     Velocity::from_linear(Vec3::new(
                    //         new_velocity.x,
                    //         new_velocity.y,
                    //         0.0,
                    //     ))
                }
            }
            CollisionEvent::Stopped(_, _) => {}
        }
    }
}

fn track_damage(
    mut events: EventReader<CollisionEvent>,
    mut blocks: Query<&mut Damage, With<Block>>,
    ball: Query<Entity, With<Ball>>,
) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(a, b) => {}
            CollisionEvent::Stopped(a, b) => {
                if let (Ok(_), Ok(mut block_damage)) = (
                    ball.get(a.rigid_body_entity()),
                    blocks.get_mut(b.rigid_body_entity()),
                ) {
                    block_damage.0 += 1;
                } else if let (
                    Ok(_),
                    Ok(mut block_damage),
                ) = (
                    ball.get(b.rigid_body_entity()),
                    blocks.get_mut(a.rigid_body_entity()),
                ) {
                    block_damage.0 += 1;
                } else {
                }
            }
        }
    }
}

fn movement(
    input: Res<Input<KeyCode>>,
    mut paddles: Query<&mut Velocity, With<Paddle>>,
) {
    if input.pressed(KeyCode::A) {
        for mut velocity in paddles.iter_mut() {
            *velocity = Velocity::from_linear(Vec3::new(
                -500.0, 0.0, 0.0,
            ));
        }
    } else if input.pressed(KeyCode::D) {
        for mut velocity in paddles.iter_mut() {
            *velocity = Velocity::from_linear(Vec3::new(
                500.0, 0.0, 0.0,
            ));
        }
    } else {
        for mut velocity in paddles.iter_mut() {
            *velocity = Velocity::from_linear(Vec3::new(
                0.0, 0.0, 0.0,
            ));
        }
    }
}

fn powerup_gravity(
    mut powerups: Query<&mut Transform, With<Powerup>>,
) {
    for mut position in powerups.iter_mut() {
        position.translation.y -= 1.0;
    }
}

fn powerup_collisions(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    paddle: Query<Entity, With<Paddle>>,
    powerup: Query<(Entity, &Powerup)>,
    mut three_balls: EventWriter<SpawnThreeBallsEvent>,
) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(a, b) => {
                let powerup = {
                    let a_entity = a.rigid_body_entity();
                    let b_entity = b.rigid_body_entity();

                    if let (Ok(_), Ok((entity, powerup))) = (
                        paddle.get(a_entity),
                        powerup.get(b_entity),
                    ) {
                        commands
                            .entity(entity)
                            .despawn_recursive();
                        Some(powerup)
                    } else if let (
                        Ok(_),
                        Ok((entity, powerup)),
                    ) = (
                        paddle.get(b_entity),
                        powerup.get(a_entity),
                    ) {
                        commands
                            .entity(entity)
                            .despawn_recursive();
                        Some(powerup)
                    } else {
                        None
                    }
                };

                if let Some(powerup) = powerup {
                    match powerup {
                        Powerup::TripleBall => {
                            three_balls
                                .send(SpawnThreeBallsEvent);
                        }
                        Powerup::WidePaddle => {
                            dbg!(
                                "Powerup not supported yet"
                            );
                        }
                        Powerup::Gunship => {
                            dbg!(
                                "Powerup not supported yet"
                            );
                        }
                    }
                }
            }
            CollisionEvent::Stopped(_, _) => {
                // dbg!("stopped");
            }
        }
    }
}

fn three_balls_events(
    mut commands: Commands,
    mut events: EventReader<SpawnThreeBallsEvent>,
    ball: Query<
        (Entity, &Velocity, &Transform),
        With<Ball>,
    >,
) {
    if let Some((_, velocity, transform)) =
        ball.iter().next()
    {
        for _ in events.iter() {
            for i in 0..2 {
                let new_velocity = {
                    let x_diff = if i % 2 == 1 {
                        -10.0
                    } else {
                        10.0
                    };

                    // a^2 + b^2 = c^2
                    let optimal_velocity: f32 =
                        100.0 * 100.0 + 400.0 * 400.0;
                    let c = optimal_velocity.sqrt();

                    // TODO: Jacob says this `10` might need
                    // to be a function of the paddle width
                    let normalized = Vec2::new(
                        x_diff * 7.5,
                        velocity.linear.y,
                    )
                    .normalize();

                    // expand normalized parts back out into
                    // full magnitude
                    let new_velocity = normalized * c;

                    Velocity::from_linear(Vec3::new(
                        new_velocity.x,
                        new_velocity.y,
                        0.0,
                    ))
                };

                commands.add(SpawnBall {
                    velocity: new_velocity,
                    transform: transform.clone(),
                })
            }
        }
    }
}
