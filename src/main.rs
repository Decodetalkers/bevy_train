use std::f32::consts::PI;

use bevy::{
    math::bounding::{BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
};

use bevy::math::bounding::Aabb2d;
use rand::Rng;

const CAT_LEN: f32 = 40.0;

const CAT_SIZE: Vec3 = Vec3::from_array([40.0, 40.0, 0.0]);
//const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);

const WALL_THICKNESS: f32 = 10.0;
// x coordinates
const LEFT_WALL: f32 = -900.;
const RIGHT_WALL: f32 = 900.;
// y coordinates
const BOTTOM_WALL: f32 = -500.;
const TOP_WALL: f32 = 500.;

const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

const MIRROR_SIZE: Vec3 = Vec3::from_array([200.0, 40.0, 0.0]);

#[derive(Default, Debug, Resource)]
struct PlayerState {
    x: f32,
    y: f32,
    mirror_exist: bool,
}

#[derive(Debug, Component)]
struct CenterPlayer;

//#[derive(Debug, Component)]
//struct SpeedPanel;
//
//#[derive(Debug, Component)]
//struct PosPanel;

//#[derive(Event, Default)]
//struct CollisionEvent;

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct Mirror;

#[derive(Bundle)]
struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

/// Which side of the arena is this wall located on?
enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl WallBundle {
    // This "builder method" allows us to reuse logic across our wall entities,
    // making our code easier to read and less prone to bugs when we change the logic
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                    // This is used to determine the order of our sprites
                    translation: location.position().extend(0.0),
                    // The z-scale of 2D objects must always be 1.0,
                    // or their ordering will be affected in surprising ways.
                    // See https://github.com/bevyengine/bevy/issues/4149
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

fn roate(mut query: Query<&mut Transform, With<CenterPlayer>>, timer: Res<Time>) {
    let mut player_trans = query.single_mut();
    player_trans.rotate_z(5.0 * timer.delta_secs());
}

fn handle_move(
    mut query: Query<&mut Transform, With<CenterPlayer>>,
    timer: Res<Time<Fixed>>,
    state: Res<PlayerState>,
) {
    let mut player_trans = query.single_mut();

    player_trans.translation.x += state.x * timer.delta().as_secs_f32();
    player_trans.translation.y += state.y * timer.delta().as_secs_f32();
}

fn generate_mirror(mut commands: Commands, mut state: ResMut<PlayerState>) {
    if state.mirror_exist {
        return;
    }

    state.mirror_exist = true;
    let mut rng = rand::thread_rng();

    let pos_x: f32 = rng.gen_range(-900.0..900.0);
    let pos_y: f32 = rng.gen_range(-500.0..500.0);
    let roat_z: f32 = rng.gen_range(0. ..2. * PI);
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3 {
                    x: pos_x,
                    y: pos_y,
                    z: 0.0,
                },
                rotation: Quat::from_rotation_z(roat_z),
                scale: MIRROR_SIZE,
                ..default()
            },
            sprite: Sprite { ..default() },
            ..default()
        },
        Mirror,
        Collider,
    ));

    let pos_x: f32 = rng.gen_range(-800.0..800.0);
    let pos_y: f32 = rng.gen_range(-300.0..300.0);
    let roat_z: f32 = rng.gen_range(0. ..2. * PI);
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3 {
                    x: pos_x,
                    y: pos_y,
                    z: 0.0,
                },
                rotation: Quat::from_rotation_z(roat_z),
                scale: MIRROR_SIZE,
                ..default()
            },
            sprite: Sprite { ..default() },
            ..default()
        },
        Mirror,
        Collider,
    ));
}

fn setup(mut commands: Commands, _assert_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                scale: CAT_SIZE,
                ..default()
            },
            sprite: Sprite { ..default() },
            ..default()
        },
        CenterPlayer,
    ));

    //commands
    //    .spawn(
    //        TextBundle {
    //            text: Text {
    //                sections: vec![
    //                    TextSection {
    //                        value: "X:".to_string(),
    //                        style: TextStyle {
    //                            font_size: 50.0,
    //                            color: TEXT_COLOR,
    //                            ..default()
    //                        },
    //                    },
    //                    TextSection {
    //                        value: "Y:".to_string(),
    //                        style: TextStyle {
    //                            font_size: 50.0,
    //                            color: TEXT_COLOR,
    //                            ..default()
    //                        },
    //                    },
    //                ],
    //                alignment: TextAlignment::Left,
    //                ..Default::default()
    //            },
    //            ..default()
    //        }
    //        .with_style(Style {
    //            position_type: PositionType::Absolute,
    //            top: Val::Px(10.0),
    //            left: Val::Px(10.0),
    //            ..default()
    //        }),
    //    )
    //    .insert(SpeedPanel);

    //commands
    //    .spawn(
    //        TextBundle {
    //            text: Text {
    //                sections: vec![
    //                    TextSection {
    //                        value: "PosX:".to_string(),
    //                        style: TextStyle {
    //                            font_size: 50.0,
    //                            color: TEXT_COLOR,
    //                            ..default()
    //                        },
    //                    },
    //                    TextSection {
    //                        value: "PosY:".to_string(),
    //                        style: TextStyle {
    //                            font_size: 50.0,
    //                            color: TEXT_COLOR,
    //                            ..default()
    //                        },
    //                    },
    //                ],

    //                alignment: TextAlignment::Left,
    //                ..Default::default()
    //            },
    //            ..default()
    //        }
    //        .with_style(Style {
    //            position_type: PositionType::Absolute,
    //            top: Val::Px(10.0),
    //            right: Val::Px(10.0),
    //            ..default()
    //        }),
    //    )
    //    .insert(PosPanel);

    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));
}

//fn handle_speedtext(state: Res<PlayerState>, mut query: Query<&mut Text, With<SpeedPanel>>) {
//    let mut text = query.single_mut();
//    text.sections[0].value = format!("X: {} ", state.x);
//    text.sections[1].value = format!("Y: {} ", state.y);
//}
//
//fn handle_postext(
//    player_query: Query<&Transform, With<CenterPlayer>>,
//    mut query: Query<&mut Text, With<PosPanel>>,
//) {
//    let translation = player_query.single().translation;
//    let mut text = query.single_mut();
//    text.sections[0].value = format!("PosX: {} ", translation.x);
//    text.sections[1].value = format!("PosY: {} ", translation.y);
//}

fn handle_state_update(mut state: ResMut<PlayerState>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        if state.x.abs() < 500. || state.x > -500.0 {
            state.x -= 50.0;
        }
    }
    if keyboard_input.pressed(KeyCode::AltRight) {
        if state.x.abs() < 500. || state.x < 500.0 {
            state.x += 50.0;
        }
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        if state.y.abs() < 500. || state.y > -500.0 {
            state.y -= 50.0;
        }
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        if state.y.abs() < 500. || state.y < 500.0 {
            state.y += 50.0;
        }
    }
}

fn check_collider(
    mut commands: Commands,
    mut state: ResMut<PlayerState>,
    query: Query<&Transform, With<CenterPlayer>>,
    collider_query: Query<(&Transform, Option<&Mirror>), With<Collider>>,
    mirror_query: Query<Entity, With<Mirror>>,
) {
    let player_trans = query.single().translation;
    let scale = query.single().scale;
    let mut delete_mirror = false;
    for (transform, mirror_if) in &collider_query {
        let collision = ball_collision(
            BoundingCircle::new(player_trans.truncate(), CAT_LEN / 2.0),
            Aabb2d::new(
                transform.translation.truncate(),
                transform.scale.truncate() / 2.0,
            ),
        );

        if let Some(coll) = collision {
            if mirror_if.is_some() {
                delete_mirror = true;
            }
            match coll {
                Collision::Left | Collision::Right => state.x = -state.x,
                Collision::Top | Collision::Bottom => state.y = -state.y,
            }
            continue;
        }
    }
    if delete_mirror {
        for entity in &mirror_query {
            commands.entity(entity).despawn();
        }
        state.mirror_exist = false;
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

// Returns `Some` if `ball` collides with `bounding_box`.
// The returned `Collision` is the side of `bounding_box` that `ball` hit.
fn ball_collision(ball: BoundingCircle, bounding_box: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&bounding_box) {
        return None;
    }

    let closest = bounding_box.closest_point(ball.center());
    let offset: Vec2 = ball.center() - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}

fn main() {
    App::new()
        .insert_resource(PlayerState::default())
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                roate,
                check_collider,
                generate_mirror,
                handle_state_update,
                handle_move.after(handle_state_update),
            ),
        )
        //.add_systems(Update, (handle_speedtext, handle_postext))
        .run();
}
