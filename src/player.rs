use crate::theme;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
        app.add_systems(Update, update);
    }
}

#[derive(Component)]
pub(crate) struct Player;

#[derive(Component)]
struct Velocity(Vec2);

pub(crate) const PLAYER_RADIUS: f32 = 20.0;

fn spawn(
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    let start_pos: Vec2 = Vec2::new(-500.0, 300.0);
    commands.spawn((
        Player,
        Velocity(Vec2::ZERO),
        Mesh2d(mesh.add(Circle::new(PLAYER_RADIUS))),
        MeshMaterial2d(material.add(theme::Default::WHITE)),
        Transform::from_xyz(start_pos.x, start_pos.y, 1.0),
    ));
}

fn update(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
) {
    let gravity: f32 = -1000.0;
    let jump_force: f32 = 300.0;

    for (mut transform, mut velocity) in &mut query {
        if input.just_pressed(KeyCode::Space) {
            velocity.0.y = jump_force;
        }

        velocity.0.y += gravity * time.delta_secs();
        transform.translation.y += velocity.0.y * time.delta_secs();
    }
}
