use std::f64::consts::FRAC_PI_8;

use bevy::{
    camera::ScalingMode,
    color::palettes::tailwind::*,
    input::common_conditions::input_just_pressed,
    math::{
        FloatOrd,
        bounding::{Aabb2d, BoundingCircle, RayCast2d},
    },
    prelude::*,
};

const CANVAS_SIZE: Vec2 = Vec2::new(600., 1080.);
const BRICK_SIZE: Vec2 = Vec2::new(80., 40.);
const BALL_SIZE: f32 = 10.;

fn main() -> AppExit {
    App::new()
        .insert_resource(ClearColor(Color::from(SLATE_950)))
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        // .enable_state_scoped_entities::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppState::Playing), new_game)
        .add_systems(
            OnEnter(AppState::GameOver),
            show_restart_button,
        )
        .add_systems(
            Update,
            restart_game
                .run_if(in_state(AppState::GameOver).and(
                    input_just_pressed(KeyCode::KeyR),
                )),
        )
        .add_systems(
            FixedUpdate,
            (paddle_controls, ball_movement),
        )
        .run()
}

#[derive(
    Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States,
)]
enum AppState {
    #[default]
    GameOver,
    Playing,
}

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Brick;

#[derive(Component)]
struct RespawnBallArea;

#[derive(Debug, Component)]
struct Wall(Plane2d);

#[derive(Debug, Component)]
struct Velocity(Vec2);

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            // scaling_mode: ScalingMode::FixedVertical {
            //     viewport_height: 1080.,
            // },
            ..OrthographicProjection::default_2d()
        }),
    ));

    commands.spawn((
        Wall(Plane2d::new(Vec2::X)),
        Transform::from_xyz(-300., 0., 0.),
    ));
    commands.spawn((
        Wall(Plane2d::new(Vec2::NEG_X)),
        Transform::from_xyz(300., 0., 0.),
    ));
    commands.spawn((
        Wall(Plane2d::new(Vec2::Y)),
        Transform::from_xyz(
            0.,
            -(CANVAS_SIZE.y / 2. - 20.),
            0.,
        ),
    ));
    commands.spawn((
        Wall(Plane2d::new(Vec2::NEG_Y)),
        Transform::from_xyz(
            0.,
            CANVAS_SIZE.y / 2. - 20.,
            0.,
        ),
    ));

    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(
                CANVAS_SIZE.x,
                CANVAS_SIZE.y - 40.,
            )),
            color: Color::from(SKY_800),
            ..default()
        },
        Transform::from_xyz(0., 0., -1.0),
    ));

    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(
                CANVAS_SIZE.x,
                CANVAS_SIZE.y / 8. - 20.,
            )),
            color: Color::from(SKY_400).with_alpha(0.4),
            ..default()
        },
        Transform::from_xyz(
            0.,
            -(CANVAS_SIZE.y / 2. - 10.)
                + CANVAS_SIZE.y / 8. / 2.,
            -1.0,
        ),
        RespawnBallArea,
        // Collider::rectangle(
        //     CANVAS_SIZE.x,
        //     CANVAS_SIZE.y / 8. - 20.,
        // ),
    ));
}

fn new_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(200.0, 20.0))),
        MeshMaterial2d(
            materials.add(Color::from(SLATE_950)),
        ),
        Transform::from_xyz(
            0.0,
            -(CANVAS_SIZE.y * (3. / 8.)),
            0.0,
        ),
        Paddle,
        DespawnOnExit(AppState::Playing),
        // Collider for paddle
    ));

    commands.spawn((
        Ball,
        Visibility::Hidden,
        Velocity(Vec2::new(-200., -400.)),
        Mesh2d(meshes.add(Circle::new(BALL_SIZE))),
        MeshMaterial2d(
            materials.add(Color::from(SLATE_950)),
        ),
        Transform::from_xyz(0.0, 0.0, 0.0),
        DespawnOnExit(AppState::Playing),
        // Collider::circle(10.),
        children![(
            Mesh2d(meshes.add(Circle::new(9.))),
            MeshMaterial2d(materials.add(Color::WHITE)),
        )],
    ));
    // .observe(
    //     |trigger: On<OnCollisionStart>,
    //      mut commands: Commands,
    //      bricks: Query<(), With<Brick>>,
    //      respawn_areas: Query<
    //         (),
    //         With<RespawnBallArea>,
    //     >,
    //      mut next_state: ResMut<
    //         NextState<AppState>,
    //     >| {
    //         if bricks.contains(trigger.event().0) {
    //             commands
    //                 .entity(trigger.event().0)
    //                 .despawn();
    //         }
    //         if respawn_areas.contains(trigger.event().0)
    //         {
    //             next_state.set(AppState::GameOver);
    //         }
    //     },
    // );

    let num_bricks_per_row = 6;
    let rows = 4;
    let color = Oklcha::from(SKY_400);
    for row in 0..rows {
        for i in 0..num_bricks_per_row {
            let current_color = color.with_hue(
                ((row + i) % 8) as f32
                    * (num_bricks_per_row * rows) as f32,
            );
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::new(
                    BRICK_SIZE.x,
                    BRICK_SIZE.y,
                ))),
                MeshMaterial2d(
                    materials.add(
                        Color::from(current_color)
                            .with_alpha(0.2),
                    ),
                ),
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
                Brick,
                DespawnOnExit(AppState::Playing),
                // Collider::rectangle(
                //     brick_size.x,
                //     brick_size.y,
                // ),
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
                )],
            ));
        }
    }
}

const PADDLE_SPEED: f32 = 400.0;
fn paddle_controls(
    input: Res<ButtonInput<KeyCode>>,
    mut paddles: Query<&mut Transform, With<Paddle>>,
    time: Res<Time>,
) {
    for mut transform in &mut paddles {
        if input.pressed(KeyCode::KeyA) {
            // if colliding with wall, don't move left
            transform.translation.x -=
                PADDLE_SPEED * time.delta_secs();
        } else if input.pressed(KeyCode::KeyD) {
            // if colliding with wall, don't move right
            transform.translation.x +=
                PADDLE_SPEED * time.delta_secs();
        }
    }
}

fn show_restart_button(mut commands: Commands) {
    commands.spawn((
        Text::new("Press R to Restart Game"),
        TextFont::from_font_size(67.0),
        TextColor(SLATE_50.into()),
        DespawnOnExit(AppState::GameOver),
    ));
}

fn restart_game(
    mut next_state: ResMut<NextState<AppState>>,
) {
    next_state.set(AppState::Playing);
}

fn ball_movement(
    mut balls: Query<
        (&mut Transform, &mut Velocity),
        With<Ball>,
    >,
    walls: Query<(&Wall, &Transform), Without<Ball>>,
    bricks: Query<&Transform, (Without<Ball>, With<Brick>)>,
    paddles: Query<(&Paddle, &Transform), Without<Ball>>,
    time: Res<Time>,
    mut gizmos: Gizmos,
) {
    for (mut transform, mut velocity) in &mut balls {
        gizmos.circle_2d(
            transform.translation.xy(),
            2.,
            Color::WHITE,
        );
        // the Direction the ball is moving in
        let ball_direction = Dir2::new(velocity.0).unwrap();
        // a ray that casts infinitely in the direction
        // the ball is moving
        let new_ray = Ray2d::new(
            transform.translation.xy(),
            ball_direction,
        );

        // how far the ball is going to go this frame
        // represented as a vec2
        let ball_movement_this_frame =
            velocity.0 * time.delta_secs();

        // for each wall, check if we're going to hit it this frame
        for (wall, origin) in walls {
            if let Some(hit_distance) = new_ray
                .intersect_plane(
                    origin.translation.xy(),
                    wall.0,
                )
                && hit_distance
                    <= ball_movement_this_frame.length()
            {
                // todo: travel some length towards wall, then some away from the reflected hit

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
            new_ray,
            ball_movement_this_frame.length(),
        );

        // for each brick, check if we're going to hit it this frame
        // This *could* be a check against *all* bricks, where we
        // then take the minimum distance instead.
        for (index, origin) in bricks.iter().enumerate() {
            let brick_collider = Aabb2d::new(
                origin.translation.xy(),
                BRICK_SIZE / 2.,
            );

            if let Some(hit_distance) = ball_cast
                .aabb_intersection_at(&brick_collider)
                && hit_distance
                    <= ball_movement_this_frame.length()
            {
                // figure out which aabb side we hit using planes
                let (hit_normal, _) = [
                    (
                        Plane2d::new(Vec2::NEG_Y),
                        Vec2::new(
                            origin.translation.x,
                            brick_collider.min.y,
                        ),
                    ),
                    (
                        Plane2d::new(Vec2::Y),
                        Vec2::new(
                            origin.translation.x,
                            brick_collider.max.y,
                        ),
                    ),
                    (
                        Plane2d::new(Vec2::NEG_X),
                        Vec2::new(
                            brick_collider.min.x,
                            origin.translation.y,
                        ),
                    ),
                    (
                        Plane2d::new(Vec2::X),
                        Vec2::new(
                            brick_collider.max.x,
                            origin.translation.y,
                        ),
                    ),
                ]
                .into_iter()
                .filter_map(|(plane, location)| {
                    new_ray
                        .intersect_plane(location, plane)
                        .map(|hit| (plane.normal, hit))
                })
                .min_by(
                    |(_, distance_a), (_, distance_b)| {
                        FloatOrd(*distance_a)
                            .cmp(&FloatOrd(*distance_b))
                    },
                )
                .unwrap();

                info!(
                    ?index,
                    ?hit_normal,
                    ?velocity,
                    "hit"
                );
                // todo: travel some length towards wall, then some away from the reflected hit

                // travel to the "hit point" on the Aabb2d border
                velocity.0 =
                    velocity.0.reflect(*hit_normal);
                // return because we only want to handle one brick collision.
                // If we don't return (or take another approach here), then
                // we can end up in a situation where the infinitely small
                // ball is reflected back and forth between two bricks with
                // opposite normals in the same frame, resulting in an
                // "unmoving ball".
                return;
            }
        }
        // Since all hits cause a `return`, this logic should only
        // run if we *don't* hit anything.
        // If we didn't hit something, then move the ball forward
        // in its velocity direction
        transform.translation +=
            (ball_movement_this_frame).extend(0.);
    }
}
