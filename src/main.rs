use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Auto Battle".into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_scene)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 相机
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 14.0, 18.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    // 方向光
    commands.spawn((
        DirectionalLight {
            illuminance: 10_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    // 地面
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(18.0, 6.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.22, 0.38, 0.24),
            perceptual_roughness: 1.0,
            ..default()
        })),
        Transform::default(),
    ));
}
