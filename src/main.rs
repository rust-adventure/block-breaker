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
    render::settings::{WgpuFeatures, WgpuSettings},
    sprite::Anchor,
};
use bevy_hanabi::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

fn main() {
    let mut options = WgpuSettings::default();
    options.features.set(
        WgpuFeatures::VERTEX_WRITABLE_STORAGE,
        true,
    );

    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(Board::new(11, 28))
        .insert_resource(options)
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(UiPlugin)
        .add_plugin(AssetsPlugin)
        .add_plugin(ShapePlugin)
        .add_plugin(HanabiPlugin)
        .insert_resource(ClearColor(Color::rgb(
            0.5, 0.5, 0.5,
        )))
        .add_loopless_state(STARTING_GAME_STATE)
        .add_plugin(ScorePlugin)
        .add_event::<SpawnThreeBallsEvent>()
        .add_startup_system(setup)
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Playing)
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
) {
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 2.0,
            ..default()
        },
        transform: Transform::from_xyz(
            board.physical.x / 2.0,
            board.physical.y / 2.0,
            1000.0,
        ),
        ..default()
    });

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
}

const BALL_RADIUS: f32 = 20.05;

fn spawn_new_game(
    mut commands: Commands,
    _images: Res<ImageAssets>,
    board: Res<Board>,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    let shape = shapes::Circle {
        radius: 10.0,
        ..Default::default()
    };

    // commands.add(SpawnBall{
    //     velocity: todo!(),
    //     transform: todo!(),
    // });
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: bevy_prototype_lyon::prelude::FillMode::color(Color::WHITE),
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
        .insert(Velocity::linear(Vec2::new(
            100.0, 400.0
        )))
        .insert(Ball)
        .insert(GravityScale(0.0))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(ActiveEvents::COLLISION_EVENTS);

    let paddle_id = commands
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
        // .insert(Restitution {
        //     coefficient: 1.0,
        //     combine_rule: CoefficientCombineRule::Min,
        // })
        // .insert(Friction {
        //     coefficient: 0.0,
        //     combine_rule: CoefficientCombineRule::Min,
        // })
        // .insert(
        //     ColliderMassProperties::Density,
        // )
        .insert(RigidBody::KinematicPositionBased)
        .insert(KinematicCharacterController {
            // filter_flags: QueryFilterFlags::EXCLUDE_FIXED,
            // filter_groups: Some(),
            // apply_impulse_to_dynamic_bodies: true,
            ..default()
        })
        .insert(Collider::cuboid(100.0, 10.0))
        // .insert(Velocity::linear(Vec2::new(0.0, 0.0)))
        // .insert(Collisions::default())
        .insert(Paddle)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .id();

    dbg!(paddle_id);
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
            fill_mode: bevy_prototype_lyon::prelude::FillMode::color(Color::rgba(
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
        .insert(RigidBody::Fixed)
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
        .insert(Collider::polyline(
            vec![
                Vect::new(0.0, 0.0),
                Vect::new(board.physical.x, 0.0),
                Vect::new(
                    board.physical.x,
                    board.physical.y,
                ),
                Vect::new(0.0, board.physical.y),
            ],
            Some(vec![[0, 1], [1, 2], [2, 3], [3, 0]]),
        ))
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
                / 2.0
                // -10.0 is a magic number meant to move the despawn area
                // beneath the paddle completely so there are no possible paddle/despawn 
                // collision events.
                - 10.0,
            0.0,
        ))
        .insert(Sensor)
        .insert(Collider::cuboid(
            board.physical.x / 2.0,
            (board.physical.y / 2.0
                + board.u8_cell_to_physical(
                    3,
                    board::Axis::Y,
                ))
                / 2.0,
        ))
        .insert(DespawnArea);

    commands.add(SpawnLevel { level: 1 });

    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1.0, 0.0, 0.0, 1.0));
    gradient.add_key(1.0, Vec4::new(1.0, 0.0, 1.0, 0.0));

    let spawner = Spawner::once(30.0.into(), false);
    let effect = effects.add(
        EffectAsset {
            name: "Impact".into(),
            capacity: 32768,
            spawner,
            ..Default::default()
        }
        .init(PositionCircleModifier {
            axis: Vec3::Z,
            radius: BALL_RADIUS,
            speed: 10.2.into(),
            dimension: ShapeDimension::Surface,
            ..Default::default()
        })
        .render(SizeOverLifetimeModifier {
            gradient: Gradient::constant(Vec2::splat(
                10.05,
            )),
        })
        .render(ColorOverLifetimeModifier { gradient }),
    );

    commands
        .spawn_bundle(
            ParticleEffectBundle::new(effect)
                .with_spawner(spawner),
        )
        .insert(Name::new("effect"));
}

fn despawn_area_collisions(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    despawn_area: Query<Entity, With<DespawnArea>>,
    mut ball: Query<Entity, With<Ball>>,
) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(a, b, _) => {
                // info!(?a, ?b, "despawn_event");
                if let (Ok(entity), Ok(_wall)) =
                    (ball.get_mut(*a), despawn_area.get(*b))
                {
                    commands
                        .entity(entity)
                        .despawn_recursive();
                } else if let (Ok(entity), Ok(_wall)) =
                    (ball.get_mut(*b), despawn_area.get(*a))
                {
                    commands
                        .entity(entity)
                        .despawn_recursive();
                }
            }
            CollisionEvent::Stopped(_, _, _) => {
                // dbg!("stopped");
            }
        }
    }
}
fn ball_collisions(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    mut balls: Query<
        (&mut Velocity, &Transform),
        (With<Ball>, Without<Paddle>),
    >,
    paddles: Query<
        (Entity, &Transform),
        (With<Paddle>, Without<Ball>),
    >,
    mut effect: Query<
        (&mut ParticleEffect, &mut Transform),
        (Without<Ball>, Without<Paddle>),
    >,
) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(a, b, _) => {
                // info!(?a, ?b, "ball_collision");
                let mut ball = if let Ok(a) =
                    balls.get_mut(*a)
                {
                    Some(a)
                } else if let Ok(b) = balls.get_mut(*b) {
                    Some(b)
                } else {
                    None
                };

                // spawn particle at ball location
                if let Some((_velocity, ball_transform)) =
                    &ball
                {
                    let (mut effect, mut effect_transform) =
                        effect.single_mut();
                    effect_transform.translation =
                        ball_transform.translation;
                    effect_transform.translation.z = 10.0;
                    // Spawn the particles
                    effect.maybe_spawner().unwrap().reset();
                }

                let paddle = if let Ok(a) = paddles.get(*a)
                {
                    Some(a)
                } else if let Ok(b) = paddles.get(*b) {
                    Some(b)
                } else {
                    None
                };
                if let (
                    Some((velocity, ball_transform)),
                    Some((_, paddle_transform)),
                ) = (&mut ball, paddle)
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
                        velocity.linvel.y,
                    )
                    .normalize();

                    // expand normalized parts back out into
                    // full magnitude
                    let new_velocity = normalized * c;

                    **velocity =
                        Velocity::linear(Vec2::new(
                            new_velocity.x,
                            new_velocity.y.abs(),
                        ))
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
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
            CollisionEvent::Started(_a, _b, _) => {}
            CollisionEvent::Stopped(a, b, _) => {
                if let (Ok(_), Ok(mut block_damage)) =
                    (ball.get(*a), blocks.get_mut(*b))
                {
                    block_damage.0 += 1;
                } else if let (
                    Ok(_),
                    Ok(mut block_damage),
                ) =
                    (ball.get(*b), blocks.get_mut(*a))
                {
                    block_damage.0 += 1;
                } else {
                }
            }
        }
    }
}

const PADDLE_SPEED: f32 = 5.0;
fn movement(
    input: Res<Input<KeyCode>>,
    mut controllers: Query<
        &mut KinematicCharacterController,
        With<Paddle>,
    >,
) {
    for mut controller in controllers.iter_mut() {
        if input.pressed(KeyCode::A) {
            controller.translation = match controller
                .translation
            {
                Some(mut vector) => {
                    vector.x = -PADDLE_SPEED;
                    Some(vector)
                }
                None => Some(Vec2::new(-PADDLE_SPEED, 0.0)),
            }
        } else if input.pressed(KeyCode::D) {
            controller.translation = match controller
                .translation
            {
                Some(mut vector) => {
                    vector.x = PADDLE_SPEED;
                    Some(vector)
                }
                None => Some(Vec2::new(PADDLE_SPEED, 0.0)),
            }
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
    rapier_context: Res<RapierContext>,
    powerups: Query<(Entity, &Powerup)>,
    paddle: Query<Entity, With<Paddle>>,
    mut three_balls: EventWriter<SpawnThreeBallsEvent>,
) {
    let paddle = paddle.single();
    for (powerup_sensor, powerup) in powerups.iter() {
        match rapier_context
            .intersection_pair(paddle, powerup_sensor)
        {
            Some(_) => {
                commands
                    .entity(powerup_sensor)
                    .despawn_recursive();

                match powerup {
                    Powerup::TripleBall => {
                        three_balls
                            .send(SpawnThreeBallsEvent);
                    }
                    Powerup::WidePaddle => {
                        dbg!("Powerup not supported yet");
                    }
                    Powerup::Gunship => {
                        dbg!("Powerup not supported yet");
                    }
                    Powerup::Sticky => {
                        dbg!("Powerup not supported yet");
                    }
                    Powerup::Life => {
                        dbg!("Powerup not supported yet");
                    }
                }
            }
            None => {
                // info!("none");
            }
        };
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
                        velocity.linvel.y,
                    )
                    .normalize();

                    // expand normalized parts back out into
                    // full magnitude
                    let new_velocity = normalized * c;

                    Velocity::linear(Vec2::new(
                        new_velocity.x,
                        new_velocity.y,
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
