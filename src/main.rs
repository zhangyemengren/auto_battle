use bevy::prelude::*;

#[derive(Component)]
enum Team {
    Red,
    Blue,
}

#[derive(Component)]
struct MoveSpeed{
    units_per_second: f32,
}

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
        .add_systems(Update, move_units)
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

    // 红
    commands.spawn((
        Team::Red,
        MoveSpeed{units_per_second: 1.0},
        Mesh3d(unit_mesh.clone()),
        MeshMaterial3d(red_meterial),
        Transform::from_xyz(-6.0, 0.5, 0.0),
    ));

    // 蓝
    commands.spawn((
        Team::Blue,
        MoveSpeed{units_per_second: 1.0},
        Mesh3d(unit_mesh),
        MeshMaterial3d(blue_meterial),
        Transform::from_xyz(6.0,0.5,0.0),
    ));
}

/// 移动系统
fn move_units(time: Res<Time>, mut units: Query<(&Team, &MoveSpeed, &mut Transform)>){
    for (team, speed, mut transform) in units{
        // 距离 = 每秒速度 * 帧数花费秒数
        let distance = speed.units_per_second * time.delta_secs();
        // 移动限制 0.6 防止双方贴合，留点距离
        match team{
            Team::Red => {
                if transform.translation.x < -0.6{
                    transform.translation.x = (transform.translation.x + distance).min(-0.6);
                }
            },
            Team::Blue => {
                if transform.translation.x > 0.6 {
                    transform.translation.x = (transform.translation.x - distance).max(0.6);
                }
            }
        }
    }
}
