use bevy::prelude::*;

mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.10, 0.11, 0.15)))
        .add_plugins(player::PlayerPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
