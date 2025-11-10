use bevy::{color::palettes::basic::WHITE, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    // asset registries
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // spawn camera
    commands.spawn(Camera2d);

    // register assets (to get handle)
    let mesh = meshes.add(Rectangle::new(32., 32.));
    let material = materials.add(Color::from(WHITE));
}
