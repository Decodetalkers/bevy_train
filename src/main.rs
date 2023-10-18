use bevy::prelude::*;

const CAT_SIZE: Vec3 = Vec3::from_array([40.0, 40.0, 0.0]);

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
}

fn handle_move(mut state: ResMut<PlayerState>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Left) {
        state.x -= 50.0;
    }
    if keyboard_input.just_pressed(KeyCode::Right) {
        state.x += 50.0;
    }
    if keyboard_input.just_pressed(KeyCode::Down) {
        state.y -= 50.0;
    }
    if keyboard_input.just_pressed(KeyCode::Up) {
        state.y += 50.0;
    }
}

fn main() {
    App::new()
        .insert_resource(PlayerState::default())
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, roate)
        .add_systems(Update, handle_move)
        .run();
}
