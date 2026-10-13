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
        .add_systems(Startup, (setup_scene, setup_units))
        .run();
}

/// 设置场景
fn setup_scene(
    mut commands: Commands, // 实体
    mut meshes: ResMut<Assets<Mesh>>, // 形状
    mut materials: ResMut<Assets<StandardMaterial>>, //外观
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

/// 创建单位
fn setup_units(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 共享网格句柄
    let unit_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));

    let red_meterial = materials.add(StandardMaterial {
        base_color: Color::srgb(0.85, 0.12, 0.12),
        ..default()
    });

    let blue_meterial = materials.add(StandardMaterial {
        base_color: Color::srgb(0.12, 0.30, 0.90),
        ..default()
    });

    commands.spawn((
        Mesh3d(unit_mesh.clone()),
        MeshMaterial3d(red_meterial),
        Transform::from_xyz(-6.0, 0.5, 0.0),
    ));

    commands.spawn((
        Mesh3d(unit_mesh),
        MeshMaterial3d(blue_meterial),
        Transform::from_xyz(6.0,0.5,0.0),
    ));

}
