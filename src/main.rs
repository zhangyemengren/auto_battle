use bevy::prelude::*;

const RED_RGB: (f32, f32, f32) = (0.85, 0.12, 0.12);
const BLUE_RGB: (f32, f32, f32) = (0.12, 0.30, 0.90);
const MIN_UNIT_DISTANCE: f32 = 0.6;

#[derive(Component)]
enum Team {
    Red,
    Blue,
}

#[derive(Component)]
struct MoveSpeed {
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
        .add_systems(
            Startup,
            (setup_scene, setup_units, setup_bases, setup_field_markings),
        )
        .add_systems(Update, move_units)
        .run();
}

/// 设置场景
fn setup_scene(
    mut commands: Commands,                          // 实体
    mut meshes: ResMut<Assets<Mesh>>,                // 形状
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

    let red_material = materials.add(StandardMaterial {
        base_color: Color::srgb(RED_RGB.0, RED_RGB.1, RED_RGB.2),
        ..default()
    });

    let blue_material = materials.add(StandardMaterial {
        base_color: Color::srgb(BLUE_RGB.0, BLUE_RGB.1, BLUE_RGB.2),
        ..default()
    });

    // 红
    commands.spawn((
        Team::Red,
        MoveSpeed {
            units_per_second: 1.0,
        },
        Mesh3d(unit_mesh.clone()),
        MeshMaterial3d(red_material),
        Transform::from_xyz(-6.0, 0.5, 0.0),
    ));

    // 蓝
    commands.spawn((
        Team::Blue,
        MoveSpeed {
            units_per_second: 1.0,
        },
        Mesh3d(unit_mesh),
        MeshMaterial3d(blue_material),
        Transform::from_xyz(6.0, 0.5, 0.0),
    ));
}

/// 移动系统
fn move_units(time: Res<Time>, mut units: Query<(&Team, &MoveSpeed, &mut Transform)>) {
    for (team, speed, mut transform) in units {
        // 距离 = 每秒速度 * 帧数花费秒数
        let distance = speed.units_per_second * time.delta_secs();
        // 移动限制 MIN_UNIT_DISTANCE 防止双方贴合，留点距离
        match team {
            Team::Red => {
                if transform.translation.x < -MIN_UNIT_DISTANCE {
                    transform.translation.x =
                        (transform.translation.x + distance).min(-MIN_UNIT_DISTANCE);
                }
            }
            Team::Blue => {
                if transform.translation.x > MIN_UNIT_DISTANCE {
                    transform.translation.x =
                        (transform.translation.x - distance).max(MIN_UNIT_DISTANCE);
                }
            }
        }
    }
}

/// 基地
fn setup_bases(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let base_mesh = meshes.add(Mesh::from(Cuboid::new(1.5, 2.0, 2.0)));

    let red_material = materials.add(StandardMaterial {
        base_color: Color::srgb(RED_RGB.0, RED_RGB.1, RED_RGB.2),
        ..default()
    });

    let blue_material = materials.add(StandardMaterial {
        base_color: Color::srgb(BLUE_RGB.0, BLUE_RGB.1, BLUE_RGB.2),
        ..default()
    });
    // 红
    commands.spawn((
        Team::Red,
        Mesh3d(base_mesh.clone()),
        MeshMaterial3d(red_material),
        Transform::from_xyz(-8.0, 1.0, 0.0),
    ));
    // 蓝
    commands.spawn((
        Team::Blue,
        Mesh3d(base_mesh),
        MeshMaterial3d(blue_material),
        Transform::from_xyz(8.0, 1.0, 0.0),
    ));
}

/// 部署区
fn setup_field_markings(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let red_deploy_x = (-6.5, -3.5);
    let blue_deploy_x = (3.5, 6.5);
    let deploy_z = (-1.5, 1.5);
    let depth = deploy_z.1 - deploy_z.0;
    let width_r = red_deploy_x.1 - red_deploy_x.0;
    let width_b = blue_deploy_x.1 - blue_deploy_x.0;
    let center_r = (red_deploy_x.0 + red_deploy_x.1) / 2.0;
    let center_b = (blue_deploy_x.0 + blue_deploy_x.1) / 2.0;
    let center_z = (deploy_z.1 + deploy_z.0) / 2.0;
    commands.spawn((
        Team::Red,
        Mesh3d(meshes.add(Plane3d::default().mesh().size(width_r, depth))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.65, 0.30, 0.30),
            perceptual_roughness: 1.0,
            ..default()
        })),
        Transform::from_xyz(center_r, 0.01, center_z),
    ));

    commands.spawn((
        Team::Blue,
        Mesh3d(meshes.add(Plane3d::default().mesh().size(width_b, depth))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.30, 0.45, 0.70),
            perceptual_roughness: 1.0,
            ..default()
        })),
        Transform::from_xyz(center_b, 0.01, center_z),
    ));
    // 中心线：沿 X 方向长 14，沿 Z 方向仅宽 0.08。
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(14.0, 0.08))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.90, 0.70),
            unlit: true,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.02, 0.0),
    ));
}
