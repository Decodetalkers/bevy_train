use bevy::prelude::*;

const CAT_SIZE: Vec3 = Vec3::from_array([40.0, 40.0, 0.0]);

#[derive(Debug, Component)]
struct CenterPlayer;

fn roate(mut query: Query<&mut Transform, With<CenterPlayer>>, timer: Res<Time>) {
    let mut player_trans = query.single_mut();
    player_trans.rotate_z(5.0 * timer.delta_seconds());
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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, roate)
        .run();
}
