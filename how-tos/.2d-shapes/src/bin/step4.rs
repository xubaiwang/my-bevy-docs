use core::f32::consts::FRAC_PI_4;

use bevy::{color::palettes::basic::WHITE, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // spawn camera
    commands.spawn(Camera2d);

    // register assets (to get handle)
    let mesh = meshes.add(Rectangle::new(32., 32.));
    let material = materials.add(Color::from(WHITE));

    // spawn the mesh entity
    let mesh = Mesh2d(mesh);
    let material = MeshMaterial2d(material);

    // center, default transform
    commands.spawn((mesh.clone(), material.clone(), Transform::default()));
    // left, rotate
    commands.spawn((
        mesh.clone(),
        material.clone(),
        Transform::default()
            .with_translation(Vec3::ZERO.with_x(-200.))
            .with_rotation(Quat::from_axis_angle(Vec3::Z, FRAC_PI_4)),
    ));
    // right, scale
    commands.spawn((
        mesh.clone(),
        material.clone(),
        Transform::default()
            .with_translation(Vec3::ZERO.with_x(200.))
            .with_scale(Vec3::splat(3.)),
    ));
}
