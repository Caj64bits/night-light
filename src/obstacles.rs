use crate::theme;
use bevy::prelude::*;
use rand::RngExt;

pub struct ObstaclePlugin;

#[derive(Component)]
struct Obstacle;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Resource)]
struct ObstacleSpawnTimer(Timer);

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ObstacleSpawnTimer(Timer::from_seconds(
            2.0,
            TimerMode::Repeating,
        )));
        app.add_systems(Update, (update_pipes, move_left));
    }
}

fn spawn_pipes(
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
    center_y: f32,
) {
    let obstacle_height: f32 = 1000.0;
    let obstacle_width: f32 = 50.0;
    let spawn_x: f32 = 300.0;
    let gap_size: f32 = 150.0;

    let obstacle_speed = 200.0;
    commands.spawn((
        Obstacle,
        Velocity(Vec2::new(-obstacle_speed, 0.0)),
        Mesh2d(mesh.add(Rectangle::new(obstacle_width, obstacle_height))),
        MeshMaterial2d(material.add(theme::Default::RED)),
        Transform::from_xyz(
            spawn_x,
            center_y - (gap_size / 2.0) - (obstacle_height / 2.0),
            1.0,
        ),
    ));

    commands.spawn((
        Obstacle,
        Velocity(Vec2::new(-obstacle_speed, 0.0)),
        Mesh2d(mesh.add(Rectangle::new(obstacle_width, obstacle_height))),
        MeshMaterial2d(material.add(theme::Default::RED)),
        Transform::from_xyz(
            spawn_x,
            center_y + (gap_size / 2.0) + (obstacle_height / 2.0),
            1.0,
        ),
    ));
}

fn update_pipes(
    mut timer: ResMut<ObstacleSpawnTimer>,
    time: Res<Time>,
    commands: Commands,
    mesh: ResMut<Assets<Mesh>>,
    material: ResMut<Assets<ColorMaterial>>,
) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        let center_y = rand::rng().random_range(-150.0..=150.0);
        spawn_pipes(commands, mesh, material, center_y);
    }
}

fn move_left(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity), With<Obstacle>>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.0.x * time.delta_secs();
    }
}
