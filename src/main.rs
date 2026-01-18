use bevy::{color::palettes::tailwind::*, prelude::*};

const BALL_SIZE: f32 = 10.;

fn main() -> AppExit {
    App::new()
        .insert_resource(ClearColor(Color::from(SKY_950)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(FixedUpdate, ball_movement)
        .run()
}

#[derive(Component)]
struct Ball;

#[derive(Debug, Component)]
struct Velocity(Vec2);

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

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
}

fn ball_movement(
    mut balls: Query<
        (&mut Transform, &Velocity),
        With<Ball>,
    >,
    time: Res<Time>,
) {
    for (mut transform, velocity) in &mut balls {
        let ball_movement_this_frame =
            velocity.0 * time.delta_secs();

        transform.translation +=
            (ball_movement_this_frame).extend(0.);
    }
}
