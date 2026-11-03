use std::any::TypeId;
use std::f32::consts::FRAC_PI_2;

use bevy::{
    camera::ScalingMode,
    color::palettes::{
        basic::GREEN,
        tailwind::{NEUTRAL_300, NEUTRAL_800},
    },
    prelude::*,
};

const BOARD_SIDE: i32 = 12;
const BOARD_RADIUS: i32 = BOARD_SIDE - 1;
const HEX_RADIUS: f32 = 1.0;
const BORDER_WIDTH: f32 = 0.04;
const BOARD_MARGIN: f32 = 2.0;

/// 尖顶六边形的轴坐标，原点是棋盘中心。
#[derive(Component, Clone, Copy, Debug)]
struct HexCell {
    q: i32,
    r: i32,
}

#[derive(Resource)]
struct CellMaterials {
    normal: Handle<StandardMaterial>,
    selected: Handle<StandardMaterial>,
}

impl HexCell {
    fn center(self) -> Vec3 {
        let spacing_x = 3.0_f32.sqrt() * HEX_RADIUS;
        Vec3::new(
            spacing_x * (self.q as f32 + 0.5 * self.r as f32),
            0.0,
            1.5 * HEX_RADIUS * self.r as f32,
        )
    }
}

fn board_size() -> Vec2 {
    Vec2::new(
        3.0_f32.sqrt() * HEX_RADIUS * (2 * BOARD_RADIUS + 1) as f32,
        HEX_RADIUS * (3 * BOARD_RADIUS + 2) as f32,
    )
}

fn main() {
    App::new()
        .insert_resource(ClearColor(NEUTRAL_800.into()))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Auto Battle".into(),
                    ..default()
                }),
                ..default()
            }),
            MeshPickingPlugin,
        ))
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
    // 每个格子保留可拾取的网格，悬停时切换材质。
    let mesh = meshes.add(RegularPolygon::new(HEX_RADIUS, 6));
    let border_mesh = meshes.add(
        Annulus::new(HEX_RADIUS - BORDER_WIDTH, HEX_RADIUS)
            .mesh()
            .resolution(6),
    );
    let border_material = materials.add(StandardMaterial {
        base_color: NEUTRAL_300.into(),
        unlit: true,
        ..default()
    });
    let normal = materials.add(StandardMaterial {
        base_color: Color::NONE,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });
    let selected = materials.add(StandardMaterial {
        base_color: GREEN.into(),
        unlit: true,
        ..default()
    });

    commands.insert_resource(CellMaterials {
        normal: normal.clone(),
        selected,
    });

    for r in -BOARD_RADIUS..=BOARD_RADIUS {
        // 三个轴坐标 q、r、-q-r 都在半径范围内。
        let q_min = (-BOARD_RADIUS).max(-BOARD_RADIUS - r);
        let q_max = BOARD_RADIUS.min(BOARD_RADIUS - r);
        for q in q_min..=q_max {
            let cell = HexCell { q, r };
            commands
                .spawn((
                    cell,
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(normal.clone()),
                    Transform::from_translation(cell.center())
                        .with_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
                    children![(
                        Mesh3d(border_mesh.clone()),
                        MeshMaterial3d(border_material.clone()),
                        // 沿格面法线略微抬高边框，使高亮时轮廓仍然清晰。
                        Transform::from_xyz(0.0, 0.0, 0.01),
                        Pickable::IGNORE,
                    )],
                ))
                .observe(setup_color::<PointerOver>)
                .observe(setup_color::<PointerOut>);
        }
    }
}

fn setup_color<E: EntityEvent>(
    item: On<E>,
    cell_materials: Res<CellMaterials>,
    mut query: Query<&mut MeshMaterial3d<StandardMaterial>>,
) {
    if let Ok(mut material) = query.get_mut(item.event_target()) {
        material.0 = if TypeId::of::<E>() == TypeId::of::<PointerOver>() {
            cell_materials.selected.clone()
        } else {
            cell_materials.normal.clone()
        };
    }
}
