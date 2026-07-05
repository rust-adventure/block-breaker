use std::f32::consts::{FRAC_PI_4, PI};

use bevy::{
    camera::ScalingMode,
    color::palettes::tailwind::*,
    input::common_conditions::input_just_pressed,
    math::{
        FloatOrd,
        bounding::{Aabb2d, BoundingCircle, IntersectsVolume, RayCast2d},
    },
    prelude::*,
    sprite::Anchor,
};

const BRICK_SIZE: Vec2 = Vec2::new(80., 40.);
const CANVAS_SIZE: Vec2 = Vec2::new(1280., 720.);
const BALL_SIZE: f32 = 10.;
const DEFAULT_PADDLE_SIZE: Vec2 = Vec2::new(200.0, 20.0);
const PADDLE_SPEED: f32 = 400.0;

fn main() -> AppExit {
    App::new()
        .insert_resource(ClearColor(Color::from(SKY_950)))
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_systems(Startup, setup.spawn())
        .add_systems(OnEnter(AppState::Playing), new_game_scene.spawn())
        .add_systems(OnEnter(AppState::GameOver), restart_button.spawn())
        .add_systems(
            Update,
            restart_game
                .run_if(in_state(AppState::GameOver).and_then(input_just_pressed(KeyCode::KeyR))),
        )
        .add_systems(
            FixedUpdate,
            (paddle_controls, ball_movement, on_intersect_respawn_area),
        )
        .run()
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    GameOver,
    Playing,
}

#[derive(Component, Default, Clone)]
struct Paddle;

#[derive(Component, Default, Clone)]
struct Ball;

#[derive(SceneComponent, Default, Clone)]
#[scene(BrickProps)]
struct Brick;

#[derive(Default)]
struct BrickProps {
    color: Color,
    x: f32,
    y: f32,
}

impl Brick {
    fn scene(props: BrickProps) -> impl Scene {
        bsn! {
            // Brick
            Sprite {
                custom_size: BRICK_SIZE,
                color: {props.color.with_alpha(0.4)},
            }
            Transform::from_xyz(
                props.x,
                props.y,
                0.0,
            )
            HalfSize({BRICK_SIZE / 2.})
            DespawnOnExit::<AppState>(AppState::Playing)
            Children [(
                Mesh2d(asset_value(Rectangle::new(BRICK_SIZE.x - 2., BRICK_SIZE.y - 2.,)))
                MeshMaterial2d::<ColorMaterial>(asset_value(props.color))
                Transform::from_xyz(0., 0., 1.)
            )]
        }
    }
}

#[derive(Component, Default, Clone)]
struct RespawnBallArea;

#[derive(Debug, Component, Default, Clone)]
struct Wall(Plane2d);

#[derive(Debug, Component, Default, Clone)]
struct Velocity(Vec2);

#[derive(Debug, Component, Default, Clone)]
struct HalfSize(Vec2);

fn setup() -> impl SceneList {
    bsn_list! [
        Camera2d
        template_value(Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: CANVAS_SIZE.x + BRICK_SIZE.x,
                min_height: CANVAS_SIZE.y + BRICK_SIZE.y,
            },
            ..OrthographicProjection::default_2d()
        })),


        #LeftWall
        Wall(Plane2d::new(Vec2::X))
        Transform::from_xyz(-CANVAS_SIZE.x / 2., 0., 0.),

        #RightWall
        Wall(Plane2d::new(Vec2::NEG_X))
        Transform::from_xyz(CANVAS_SIZE.x / 2., 0., 0.),

        #BottomWall
        Wall(Plane2d::new(Vec2::Y))
        Transform::from_xyz(0., -(CANVAS_SIZE.y / 2.), 0.),

        #TopWall
        Wall(Plane2d::new(Vec2::NEG_Y))
        Transform::from_xyz(0., CANVAS_SIZE.y / 2., 0.),

        #StageBorder
        Sprite {
            custom_size: Vec2::new(
                // 4px border
                CANVAS_SIZE.x + 4.,
                CANVAS_SIZE.y + 4.,
            ),
            color: Color::from(SKY_50),
        }
        Transform::from_xyz(0., 0., -3.0),

        #Stage
        Sprite {
            custom_size: CANVAS_SIZE,
            color: Color::from(SKY_800),
        }
        Transform::from_xyz(0., 0., -2.0),

        #RespawnArea
        RespawnBallArea
        Sprite {
            custom_size: Vec2::new(
                CANVAS_SIZE.x,
                CANVAS_SIZE.y / 8. - DEFAULT_PADDLE_SIZE.y / 2.,
            ),
            color: {Color::from(SKY_500).with_alpha(0.4)},
        }
        Anchor::BOTTOM_CENTER
        Transform::from_xyz(0., -CANVAS_SIZE.y / 2., -1.0)
    ]
}

fn new_game_scene() -> impl SceneList {
    let num_bricks_per_row = 13;
    let rows = 6;
    let mut bricks = Vec::with_capacity(num_bricks_per_row * rows);
    let base_color = Oklcha::from(SKY_400);

    for row in 0..rows {
        for i in 0..num_bricks_per_row {
            let current_color =
                base_color.with_hue(((row + i) % 8) as f32 * (num_bricks_per_row * rows) as f32);
            bricks.push(bsn! {
                @Brick {
                    @color: Color::from(current_color),
                    @x: {BRICK_SIZE.x * i as f32 - BRICK_SIZE.x * num_bricks_per_row as f32 / 2. + BRICK_SIZE.x / 2.},
                    @y: {CANVAS_SIZE.y * (3. / 8.) - BRICK_SIZE.y * row as f32},
                }
            });
        }
    }

    bsn_list![
        #Paddle
        Paddle
        Sprite {
            custom_size: DEFAULT_PADDLE_SIZE,
            color: Color::from(SKY_50),
        }
        Transform::from_xyz(0.0, -CANVAS_SIZE.y * (3. / 8.), 0.0)
        DespawnOnExit::<AppState>(AppState::Playing)
        HalfSize({DEFAULT_PADDLE_SIZE / 2.}),

        #Ball
        Ball
        Velocity(Vec2::new(-200., -400.))
        Mesh2d(asset_value(Circle::new(BALL_SIZE)))
        MeshMaterial2d::<ColorMaterial>(asset_value(Color::from(SLATE_950)))
        Transform::from_xyz(0.0, 0.0, 0.0)
        DespawnOnExit::<AppState>(AppState::Playing)
        Children [
            Mesh2d(asset_value(Circle::new(BALL_SIZE - 1.)))
            MeshMaterial2d::<ColorMaterial>(asset_value(Color::WHITE))
            Transform::from_xyz(0., 0., 1.)
        ],

        {bricks}
    ]
}

fn paddle_controls(
    input: Res<ButtonInput<KeyCode>>,
    mut paddles: Query<&mut Transform, With<Paddle>>,
    time: Res<Time>,
) {
    for mut transform in &mut paddles {
        if input.pressed(KeyCode::KeyA) {
            // if colliding with wall, don't move left
            transform.translation.x -= PADDLE_SPEED * time.delta_secs();
        } else if input.pressed(KeyCode::KeyD) {
            // if colliding with wall, don't move right
            transform.translation.x += PADDLE_SPEED * time.delta_secs();
        }
    }
}

fn restart_button() -> impl Scene {
    bsn! {
        Node {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: percent(100.),
            height: percent(100.),
        }
        Children [(
            Text::new("Press R to Restart Game")
            TextColor({Color::from(SLATE_50)})
            DespawnOnExit::<AppState>(AppState::GameOver)
        )]
    }
}

fn restart_game(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::Playing);
}

fn ball_movement(
    mut balls: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    walls: Query<(&Wall, &Transform), Without<Ball>>,
    aabb_colliders: Query<(Entity, &Transform, &HalfSize), Without<Ball>>,
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
        let ball_movement_this_frame = velocity.0 * time.delta_secs();
        let ball_move_distance = ball_movement_this_frame.length();

        // for each wall, check if we're going to hit it this frame
        for (wall, origin) in walls {
            if let Some(hit_distance) = ball_ray.intersect_plane(origin.translation.xy(), wall.0)
                && hit_distance <= ball_move_distance
            {
                // velocity is just the reflection of the hit
                // this is basically inverting the X or Y direction
                // to move in the opposite direction
                velocity.0 = velocity.0.reflect(wall.0.normal.as_vec2());
                return;
            }
        }

        let ball_cast = RayCast2d::from_ray(ball_ray, ball_move_distance);

        // for each brick or paddle, check if we're going to hit it this frame.
        // then take the closest hit and process it, if it exists.
        if let Some((entity, origin, brick_collider, _)) = aabb_colliders
            .iter()
            .filter_map(|(entity, origin, half_size)| {
                let brick_collider = Aabb2d::new(origin.translation.xy(), half_size.0);

                // no intersection means no hit distance
                let hit_distance = ball_cast.aabb_intersection_at(&brick_collider)?;

                Some((entity, origin, brick_collider, hit_distance))
            })
            .min_by_key(|(_, _, _, distance)| FloatOrd(*distance))
        {
            // figure out which aabb side we hit using planes
            // if we made it here, this should *always* return a result
            // because we just checked to see if we hit the aabb2d.
            let (hit_normal, _) = [
                (
                    Plane2d::new(Vec2::NEG_Y),
                    Vec2::new(origin.translation.x, brick_collider.min.y),
                ),
                (
                    Plane2d::new(Vec2::Y),
                    Vec2::new(origin.translation.x, brick_collider.max.y),
                ),
                (
                    Plane2d::new(Vec2::NEG_X),
                    Vec2::new(brick_collider.min.x, origin.translation.y),
                ),
                (
                    Plane2d::new(Vec2::X),
                    Vec2::new(brick_collider.max.x, origin.translation.y),
                ),
            ]
            .into_iter()
            .filter_map(|(plane, location)| {
                ball_ray
                    .intersect_plane(location, plane)
                    .map(|hit_distance| (plane.normal, hit_distance))
            })
            .min_by_key(|(_, distance)| FloatOrd(*distance))
            .unwrap();

            if paddles.get(entity).is_ok() {
                // paddle collision is built off of the angle between the
                // ball and the paddle.
                let direction_vector = transform.translation.xy() - origin.translation.xy();

                // most angles will be PI range, so we scale to 90deg
                // to avoid players getting stuck with a *very horizontal*
                // ball trajectory, which is just unfun to wait for
                let angle = direction_vector.normalize().to_angle();
                let linear_angle = angle.clamp(0., PI) / PI;
                let softened_angle = FRAC_PI_4.lerp(PI - FRAC_PI_4, linear_angle);
                velocity.0 = Vec2::from_angle(softened_angle).normalize() * velocity.0.length()
            } else {
                commands.entity(entity).despawn();
                velocity.0 = velocity.0.reflect(*hit_normal);
            }
            break;
        }

        // Since all hits cause a `return`, this logic should only
        // run if we *don't* hit anything.
        // If we didn't hit something, then move the ball forward
        // in its velocity direction
        transform.translation += (ball_movement_this_frame).extend(0.);
    }
}

fn on_intersect_respawn_area(
    respawn_area: Single<(&Transform, &Sprite), With<RespawnBallArea>>,
    balls: Query<&Transform, With<Ball>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for ball in &balls {
        let ball_collider = BoundingCircle::new(ball.translation.xy(), BALL_SIZE);
        // check respawn area collision
        let respawn_collider = Aabb2d::new(
            respawn_area.0.translation.xy(),
            respawn_area.1.custom_size.unwrap() / Vec2::splat(2.),
        );
        if ball_collider.intersects(&respawn_collider) {
            next_state.set(AppState::GameOver);
        }
    }
}
