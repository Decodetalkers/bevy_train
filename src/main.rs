use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

const CAT_SIZE: Vec3 = Vec3::from_array([40.0, 40.0, 0.0]);
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);

const WALL_THICKNESS: f32 = 10.0;
// x coordinates
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
// y coordinates
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;

const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

#[derive(Default, Debug, Resource)]
struct PlayerState {
    x: f32,
    y: f32,
}

#[derive(Debug, Component)]
struct CenterPlayer;

#[derive(Debug, Component)]
struct SpeedPanel;

#[derive(Debug, Component)]
struct PosPanel;

#[derive(Event, Default)]
struct CollisionEvent;

#[derive(Component)]
struct Collider;

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
    player_trans.rotate_z(5.0 * timer.delta_seconds());
}

fn handle_move(
    mut query: Query<&mut Transform, With<CenterPlayer>>,
    timer: Res<Time>,
    state: Res<PlayerState>,
) {
    let mut player_trans = query.single_mut();

    player_trans.translation.x += state.x * timer.delta_seconds();
    player_trans.translation.y += state.y * timer.delta_seconds();
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

    commands
        .spawn(
            TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "X:".to_string(),
                            style: TextStyle {
                                font_size: 50.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        },
                        TextSection {
                            value: "Y:".to_string(),
                            style: TextStyle {
                                font_size: 50.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        },
                    ],
                    alignment: TextAlignment::Left,
                    ..Default::default()
                },
                ..default()
            }
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                ..default()
            }),
        )
        .insert(SpeedPanel);

    commands
        .spawn(
            TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "PosX:".to_string(),
                            style: TextStyle {
                                font_size: 50.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        },
                        TextSection {
                            value: "PosY:".to_string(),
                            style: TextStyle {
                                font_size: 50.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        },
                    ],

                    alignment: TextAlignment::Left,
                    ..Default::default()
                },
                ..default()
            }
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                ..default()
            }),
        )
        .insert(PosPanel);

    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));
}

fn handle_speedtext(state: Res<PlayerState>, mut query: Query<&mut Text, With<SpeedPanel>>) {
    let mut text = query.single_mut();
    text.sections[0].value = format!("X: {} ", state.x);
    text.sections[1].value = format!("Y: {} ", state.y);
}

fn handle_postext(
    player_query: Query<&Transform, With<CenterPlayer>>,
    mut query: Query<&mut Text, With<PosPanel>>,
) {
    let translation = player_query.single().translation;
    let mut text = query.single_mut();
    text.sections[0].value = format!("PosX: {} ", translation.x);
    text.sections[1].value = format!("PosY: {} ", translation.y);
}

fn handle_state_update(mut state: ResMut<PlayerState>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::Left) {
        state.x -= 50.0;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        state.x += 50.0;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        state.y -= 50.0;
    }
    if keyboard_input.pressed(KeyCode::Up) {
        state.y += 50.0;
    }
}

fn check_collider(
    mut state: ResMut<PlayerState>,
    query: Query<&Transform, With<CenterPlayer>>,
    collider_query: Query<&Transform, With<Collider>>,
) {
    let player_trans = query.single().translation;
    let scale = query.single().scale;
    for transform in &collider_query {
        let collision = collide(
            player_trans,
            scale.truncate(),
            transform.translation,
            transform.scale.truncate(),
        );
        if let Some(coll) = collision {
            match coll {
                Collision::Left | Collision::Right => state.x = -state.x,
                Collision::Top | Collision::Bottom => state.y = -state.y,
                Collision::Inside => { /* do nothing */ }
            }
        }
    }
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
                handle_state_update.after(check_collider),
                handle_move.after(handle_state_update),
            ),
        )
        .add_systems(Update, (handle_speedtext, handle_postext))
        .run();
}
