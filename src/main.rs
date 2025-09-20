use avian2d::{PhysicsPlugins, prelude::*};
use bevy::{
    color::palettes::tailwind::*,
    input::common_conditions::input_just_pressed,
    prelude::*,
};

const CANVAS_SIZE: Vec2 = Vec2::new(600., 1080.);

fn main() -> AppExit {
    App::new()
        .insert_resource(ClearColor(Color::from(SLATE_950)))
        .insert_resource(DefaultFriction(Friction::new(0.)))
        .insert_resource(DefaultRestitution(
            Restitution::new(1.),
        ))
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default().set(
                PhysicsInterpolationPlugin::interpolate_all(
                ),
            ),
            PhysicsDebugPlugin::default(),
        ))
        .init_state::<AppState>()
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
        .add_systems(FixedUpdate, paddle_controls)
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

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode:
                bevy::camera::ScalingMode::FixedVertical {
                    viewport_height: 1080.,
                },
            ..OrthographicProjection::default_2d()
        }),
    ));

    commands.spawn((
        RigidBody::Static,
        Collider::half_space(Vec2::X),
        Transform::from_xyz(-300., 0., 0.),
    ));
    commands.spawn((
        RigidBody::Static,
        Collider::half_space(Vec2::NEG_X),
        Transform::from_xyz(300., 0., 0.),
    ));
    commands.spawn((
        RigidBody::Static,
        Collider::half_space(Vec2::Y),
        Transform::from_xyz(
            0.,
            -(CANVAS_SIZE.y / 2. - 20.),
            0.,
        ),
    ));
    commands.spawn((
        RigidBody::Static,
        Collider::half_space(Vec2::NEG_Y),
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
        Collider::rectangle(
            CANVAS_SIZE.x,
            CANVAS_SIZE.y / 8. - 20.,
        ),
        Sensor,
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
        RigidBody::Kinematic,
        Collider::ellipse(100., 10.),
    ));

    commands
        .spawn((
            Ball,
            Mesh2d(meshes.add(Circle::new(10.))),
            MeshMaterial2d(
                materials.add(Color::from(SLATE_950)),
            ),
            Transform::from_xyz(0.0, 0.0, 0.0),
            DespawnOnExit(AppState::Playing),
            RigidBody::Dynamic,
            Collider::circle(10.),
            GravityScale(0.),
            LinearVelocity(Vec2 { x: 100., y: -400. }),
            LockedAxes::ROTATION_LOCKED,
            CollisionEventsEnabled,
            children![(
                Mesh2d(meshes.add(Circle::new(9.))),
                MeshMaterial2d(materials.add(Color::WHITE)),
                Transform::from_xyz(0., 0., 1.)
            )],
        ))
        .observe(
            |trigger: On<CollisionStart>,
             mut commands: Commands,
             bricks: Query<(), With<Brick>>,
             respawn_areas: Query<
                (),
                With<RespawnBallArea>,
            >,
             mut next_state: ResMut<
                NextState<AppState>,
            >| {
                if bricks.contains(trigger.collider2) {
                    commands
                        .entity(trigger.collider2)
                        .despawn();
                }
                if respawn_areas.contains(trigger.collider2)
                {
                    next_state.set(AppState::GameOver);
                }
            },
        );

    let brick_size = Vec2::new(80., 40.);
    let num_bricks_per_row = 6;
    let rows = 4;
    for row in 0..rows {
        for i in 0..num_bricks_per_row {
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::new(
                    brick_size.x,
                    brick_size.y,
                ))),
                MeshMaterial2d(materials.add(
                    Color::from(SLATE_950).with_alpha(0.2),
                )),
                Transform::from_xyz(
                    brick_size.x * i as f32
                        - brick_size.x
                            * num_bricks_per_row as f32
                            / 2.
                        + brick_size.x / 2.,
                    CANVAS_SIZE.y * (3. / 8.)
                        - brick_size.y * row as f32,
                    0.0,
                ),
                Brick,
                DespawnOnExit(AppState::Playing),
                RigidBody::Static,
                Collider::rectangle(
                    brick_size.x,
                    brick_size.y,
                ),
                children![(
                    Mesh2d(meshes.add(Rectangle::new(
                        brick_size.x - 2.,
                        brick_size.y - 2.,
                    ))),
                    MeshMaterial2d(
                        materials.add(Color::from(SKY_400)),
                    ),
                )],
            ));
        }
    }
}

const PADDLE_SPEED: f32 = 200.0;
fn paddle_controls(
    input: Res<ButtonInput<KeyCode>>,
    mut paddles: Query<&mut LinearVelocity, With<Paddle>>,
) {
    for mut velocity in &mut paddles {
        if input.pressed(KeyCode::KeyA) {
            velocity.x = -PADDLE_SPEED;
        } else if input.pressed(KeyCode::KeyD) {
            velocity.x = PADDLE_SPEED;
        } else {
            velocity.x = 0.;
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
