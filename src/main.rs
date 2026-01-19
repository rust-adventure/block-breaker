use std::f32::consts::{FRAC_PI_4, PI};

use bevy::{
    camera::ScalingMode,
    color::palettes::tailwind::*,
    math::{
        FloatOrd,
        bounding::{Aabb2d, RayCast2d},
    },
    prelude::*,
};

const BALL_SIZE: f32 = 10.;
const BRICK_SIZE: Vec2 = Vec2::new(80., 40.);
const CANVAS_SIZE: Vec2 = Vec2::new(1280., 720.);
const DEFAULT_PADDLE_SIZE: Vec2 = Vec2::new(200.0, 20.0);
const PADDLE_SPEED: f32 = 400.0;

fn main() -> AppExit {
    App::new()
        .insert_resource(ClearColor(Color::from(SKY_950)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(
            FixedUpdate,
            (paddle_controls, ball_movement),
        )
        .run()
}

#[derive(Component)]
struct Ball;

#[derive(Debug, Component)]
struct Velocity(Vec2);

#[derive(Debug, Component)]
struct Wall(Plane2d);

#[derive(Component)]
struct Paddle;

#[derive(Debug, Component)]
struct HalfSize(Vec2);

#[derive(Component)]
struct Brick;

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: CANVAS_SIZE.x + BRICK_SIZE.x,
                min_height: CANVAS_SIZE.y + BRICK_SIZE.y,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

    commands.spawn((
        Ball,
        Velocity(Vec2::new(-200., -400.)),
        Mesh2d(meshes.add(Circle::new(BALL_SIZE))),
        MeshMaterial2d(
            materials.add(Color::from(SLATE_950)),
        ),
        Transform::from_xyz(0.0, 0.0, 0.0),
        children![(
            Mesh2d(meshes.add(Circle::new(BALL_SIZE - 1.))),
            MeshMaterial2d(materials.add(Color::WHITE)),
            Transform::from_xyz(0., 0., 1.)
        )],
    ));

    commands.spawn((
        Wall(Plane2d::new(Vec2::X)),
        Transform::from_xyz(-CANVAS_SIZE.x / 2., 0., 0.),
    ));
    commands.spawn((
        Wall(Plane2d::new(Vec2::NEG_X)),
        Transform::from_xyz(CANVAS_SIZE.x / 2., 0., 0.),
    ));
    commands.spawn((
        Wall(Plane2d::new(Vec2::Y)),
        Transform::from_xyz(0., -CANVAS_SIZE.y / 2., 0.),
    ));
    commands.spawn((
        Wall(Plane2d::new(Vec2::NEG_Y)),
        Transform::from_xyz(0., CANVAS_SIZE.y / 2., 0.),
    ));

    // stage area, with border
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(
                // 4px border
                CANVAS_SIZE.x + 4.,
                CANVAS_SIZE.y + 4.,
            )),
            color: Color::from(SKY_50),
            ..default()
        },
        Transform::from_xyz(0., 0., -3.0),
    ));
    commands.spawn((
        Sprite {
            custom_size: Some(CANVAS_SIZE),
            color: Color::from(SKY_800),
            ..default()
        },
        Transform::from_xyz(0., 0., -2.0),
    ));

    commands.spawn((
        Sprite {
            custom_size: Some(DEFAULT_PADDLE_SIZE),
            color: SKY_50.into(),
            ..default()
        },
        Transform::from_xyz(
            0.0,
            -CANVAS_SIZE.y * (3. / 8.),
            0.0,
        ),
        Paddle,
        HalfSize(DEFAULT_PADDLE_SIZE / 2.),
    ));

    let num_bricks_per_row = 13;
    let rows = 6;
    let base_color = Oklcha::from(SKY_400);
    for row in 0..rows {
        for i in 0..num_bricks_per_row {
            let current_color = base_color.with_hue(
                ((row + i) % 8) as f32
                    * (num_bricks_per_row * rows) as f32,
            );
            commands.spawn((
                Brick,
                Sprite {
                    custom_size: Some(BRICK_SIZE),
                    color: Color::from(current_color)
                        .with_alpha(0.4),
                    ..default()
                },
                Transform::from_xyz(
                    BRICK_SIZE.x * i as f32
                        - BRICK_SIZE.x
                            * num_bricks_per_row as f32
                            / 2.
                        + BRICK_SIZE.x / 2.,
                    CANVAS_SIZE.y * (3. / 8.)
                        - BRICK_SIZE.y * row as f32,
                    0.0,
                ),
                HalfSize(BRICK_SIZE / 2.),
                children![(
                    Mesh2d(meshes.add(Rectangle::new(
                        BRICK_SIZE.x - 2.,
                        BRICK_SIZE.y - 2.,
                    ))),
                    MeshMaterial2d(
                        materials.add(Color::from(
                            current_color
                        )),
                    ),
                    Transform::from_xyz(0., 0., 1.)
                )],
            ));
        }
    }
}

fn ball_movement(
    mut balls: Query<
        (&mut Transform, &mut Velocity),
        With<Ball>,
    >,
    walls: Query<(&Wall, &Transform), Without<Ball>>,
    aabb_colliders: Query<
        (Entity, &Transform, &HalfSize),
        Without<Ball>,
    >,
    paddles: Query<(), With<Paddle>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut transform, mut velocity) in &mut balls {
        // a ray that casts infinitely in the direction
        // the ball is moving
        let ball_ray = Ray2d::new(
            // the location of the ball
            transform.translation.xy(),
            // the Direction the ball is moving in
            Dir2::new(velocity.0).unwrap(),
        );

        // how far the ball is going to go this frame
        // represented as a vec2
        let ball_movement_this_frame =
            velocity.0 * time.delta_secs();
        let ball_move_distance =
            ball_movement_this_frame.length();

        // for each wall, check if we're going to hit it this frame
        for (wall, origin) in walls {
            if let Some(hit_distance) = ball_ray
                .intersect_plane(
                    origin.translation.xy(),
                    wall.0,
                )
                && hit_distance <= ball_move_distance
            {
                // velocity is just the reflection of the hit
                // this is basically inverting the X or Y direction
                // to move in the opposite direction
                velocity.0 = velocity
                    .0
                    .reflect(wall.0.normal.as_vec2());
                return;
            }
        }

        let ball_cast = RayCast2d::from_ray(
            ball_ray,
            ball_move_distance,
        );

        // for each brick or paddle, check if we're going to hit it this frame.
        // then take the closest hit and process it, if it exists.
        if let Some((entity, origin, aabb_collider, _)) =
            aabb_colliders
                .iter()
                .filter_map(
                    |(entity, origin, half_size)| {
                        let aabb_collider = Aabb2d::new(
                            origin.translation.xy(),
                            half_size.0,
                        );

                        // no intersection means no hit distance
                        let hit_distance = ball_cast
                            .aabb_intersection_at(
                                &aabb_collider,
                            )?;

                        Some((
                            entity,
                            origin,
                            aabb_collider,
                            hit_distance,
                        ))
                    },
                )
                .min_by_key(|(_, _, _, distance)| {
                    FloatOrd(*distance)
                })
        {
            if paddles.get(entity).is_ok() {
                let direction_vector =
                    transform.translation.xy()
                        - origin.translation.xy();

                let angle = direction_vector.to_angle();
                let linear_angle = angle.clamp(0., PI) / PI;
                let softened_angle = FRAC_PI_4
                    .lerp(PI - FRAC_PI_4, linear_angle);
                velocity.0 =
                    Vec2::from_angle(softened_angle)
                        * velocity.0.length()
            } else {
                // handle bricks!
                let (hit_normal, _) = [
                    (
                        Plane2d::new(Vec2::NEG_Y),
                        Vec2::new(
                            origin.translation.x,
                            aabb_collider.min.y,
                        ),
                    ),
                    (
                        Plane2d::new(Vec2::Y),
                        Vec2::new(
                            origin.translation.x,
                            aabb_collider.max.y,
                        ),
                    ),
                    (
                        Plane2d::new(Vec2::NEG_X),
                        Vec2::new(
                            aabb_collider.min.x,
                            origin.translation.y,
                        ),
                    ),
                    (
                        Plane2d::new(Vec2::X),
                        Vec2::new(
                            aabb_collider.max.x,
                            origin.translation.y,
                        ),
                    ),
                ]
                .into_iter()
                .filter_map(|(plane, location)| {
                    ball_ray
                        .intersect_plane(location, plane)
                        .map(|hit_distance| {
                            (plane.normal, hit_distance)
                        })
                })
                .min_by_key(|(_, distance)| {
                    FloatOrd(*distance)
                })
                .unwrap();

                commands.entity(entity).despawn();
                velocity.0 =
                    velocity.0.reflect(*hit_normal);
            }
            break;
        }

        transform.translation +=
            (ball_movement_this_frame).extend(0.);
    }
}

fn paddle_controls(
    input: Res<ButtonInput<KeyCode>>,
    mut paddles: Query<&mut Transform, With<Paddle>>,
    time: Res<Time>,
) {
    for mut transform in &mut paddles {
        if input.pressed(KeyCode::KeyA) {
            transform.translation.x -=
                PADDLE_SPEED * time.delta_secs();
        } else if input.pressed(KeyCode::KeyD) {
            transform.translation.x +=
                PADDLE_SPEED * time.delta_secs();
        }
    }
}
