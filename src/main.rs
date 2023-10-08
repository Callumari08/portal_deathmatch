use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_fps_controller::controller::*;

fn main() -> ()
{
    App::new()
    .add_plugins((DefaultPlugins, RapierPhysicsPlugin::<NoUserData>::default(), FpsControllerPlugin))
    .add_systems(Startup, setup)
    .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) -> ()
{
    // Plane
   commands.spawn(PbrBundle
    {
        mesh: meshes.add(shape::Plane::from_size(100.0).into()),
        material: materials.add(Color::rgb(0.0, 0.8, 0.0).into()),
        ..default()
    });

    // Point Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 20.0, 5.0).looking_at(Vec3::ZERO, Vec3::new(0.0, 45.0, 0.0)),
        ..default()
    });
}