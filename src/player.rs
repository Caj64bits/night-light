use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
        app.add_systems(Update, update);
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec2);

fn spawn(
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    let start_pos: Vec2 = Vec2::new(-100.0, -100.0);
    commands.spawn((
        Player,
        Velocity(Vec2::ZERO),
        Mesh2d(mesh.add(Circle::new(20.0))),
        MeshMaterial2d(material.add(Color::srgb(0.75, 0.79, 0.96))),
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
        if input.pressed(KeyCode::Space) {
            velocity.0.y = jump_force;
        }

        velocity.0.y += gravity * time.delta_secs();
        transform.translation.y += velocity.0.y * time.delta_secs();
    }
}
