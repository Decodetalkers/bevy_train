use bevy::prelude::*;

const CAT_SIZE: Vec3 = Vec3::from_array([40.0, 40.0, 0.0]);
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);

#[derive(Default, Debug, Resource)]
struct PlayerState {
    x: f32,
    y: f32,
}

#[derive(Debug, Component)]
struct CenterPlayer;

fn roate(
    mut query: Query<&mut Transform, With<CenterPlayer>>,
    timer: Res<Time>,
    state: Res<PlayerState>,
) {
    let mut player_trans = query.single_mut();
    player_trans.rotate_z(5.0 * timer.delta_seconds());

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

    commands.spawn(TextBundle {
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
        style: Style {
            position_type: PositionType::Absolute,
            ..default()
        },
        ..default()
    });
}

fn handle_text(state: Res<PlayerState>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[0].value = format!("X: {} ", state.x);
    text.sections[1].value = format!("Y: {} ", state.y);
}

fn handle_move(mut state: ResMut<PlayerState>, keyboard_input: Res<Input<KeyCode>>) {
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

fn main() {
    App::new()
        .insert_resource(PlayerState::default())
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (handle_text, roate))
        .add_systems(Update, handle_move)
        .run();
}
