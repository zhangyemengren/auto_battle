use std::f32::consts::FRAC_PI_2;

use bevy::{camera::ScalingMode, prelude::*};

const BOARD_COLUMNS: u32 = 20;
const BOARD_ROWS: u32 = 20;
const HEX_RADIUS: f32 = 1.0;
const HEX_GAP: f32 = 0.04;
const BOARD_MARGIN: f32 = 2.0;

/// 尖顶六边形采用奇数行向右错开半格的坐标。
#[derive(Component, Clone, Copy, Debug)]
struct HexCell {
    column: u32,
    row: u32,
}

impl HexCell {
    fn center(self) -> Vec3 {
        let spacing_x = 3.0_f32.sqrt() * HEX_RADIUS;
        let size = board_size();
        Vec3::new(
            spacing_x * (self.column as f32 + 0.5 * (self.row % 2) as f32)
                - (size.x - spacing_x) / 2.0,
            0.0,
            1.5 * HEX_RADIUS * self.row as f32 - (size.y - 2.0 * HEX_RADIUS) / 2.0,
        )
    }
}

fn board_size() -> Vec2 {
    Vec2::new(
        3.0_f32.sqrt() * HEX_RADIUS * (BOARD_COLUMNS as f32 + 0.5),
        HEX_RADIUS * (1.5 * (BOARD_ROWS - 1) as f32 + 2.0),
    )
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.055, 0.075, 0.09)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Auto Battle".into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_camera, setup_board))
        .run();
}

fn setup_camera(mut commands: Commands) {
    let size = board_size();
    // 正交俯视保证六边形不发生透视变形，窗口缩放时始终容纳整个棋盘。
    commands.spawn((
        Camera3d::default(),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: size.x + BOARD_MARGIN * 2.0,
                min_height: size.y + BOARD_MARGIN * 2.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(0.0, 40.0, 0.0).looking_at(Vec3::ZERO, Vec3::NEG_Z),
    ));
}

fn setup_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 共享正六边形网格和材质，略微缩小每格以露出深色网格线。
    let mesh = meshes.add(RegularPolygon::new(HEX_RADIUS - HEX_GAP / 2.0, 6));
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.22, 0.38, 0.24),
        unlit: true,
        ..default()
    });

    for row in 0..BOARD_ROWS {
        for column in 0..BOARD_COLUMNS {
            let cell = HexCell { column, row };
            commands.spawn((
                cell,
                Mesh3d(mesh.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(cell.center())
                    .with_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
            ));
        }
    }
}
