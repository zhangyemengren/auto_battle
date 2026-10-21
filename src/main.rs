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

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::{camera::CameraProjection, mesh::VertexAttributeValues};

    #[test]
    fn board_spawns_400_distinct_cells_with_shared_assets() {
        let mut app = App::new();
        app.init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .add_systems(Startup, setup_board);
        app.update();

        let world = app.world_mut();
        let mut cells = world.query::<(&HexCell, &Transform)>();
        let mut coordinates = std::collections::HashSet::new();
        for (cell, transform) in cells.iter(world) {
            assert!(cell.column < 20 && cell.row < 20);
            assert!(coordinates.insert((cell.column, cell.row)));
            assert_eq!(transform.translation, cell.center());
            assert!((transform.rotation * Vec3::Z).abs_diff_eq(Vec3::Y, 1e-5));
        }
        assert_eq!(coordinates.len(), 400);
        assert_eq!(world.resource::<Assets<Mesh>>().len(), 1);
        assert_eq!(world.resource::<Assets<StandardMaterial>>().len(), 1);
    }

    #[test]
    fn each_interior_cell_has_six_equidistant_neighbors() {
        let neighbor_distance = 3.0_f32.sqrt() * HEX_RADIUS;
        // 覆盖奇偶两种错行，以及内部全部格子。
        for row in 1..BOARD_ROWS - 1 {
            for column in 1..BOARD_COLUMNS - 1 {
                let center = HexCell { column, row }.center();
                let mut neighbors = 0;
                for other_row in row - 1..=row + 1 {
                    for other_column in column - 1..=column + 1 {
                        let other = HexCell {
                            column: other_column,
                            row: other_row,
                        }
                        .center();
                        let distance = center.distance(other);
                        if distance > 1e-5 {
                            assert!(distance >= neighbor_distance - 1e-5);
                        }
                        if (distance - neighbor_distance).abs() < 1e-5 {
                            neighbors += 1;
                        }
                    }
                }
                assert_eq!(neighbors, 6, "cell ({column}, {row})");
            }
        }
    }

    #[test]
    fn tile_mesh_is_a_regular_hexagon() {
        let mesh = Mesh::from(RegularPolygon::new(HEX_RADIUS - HEX_GAP / 2.0, 6));
        let Some(VertexAttributeValues::Float32x3(vertices)) =
            mesh.attribute(Mesh::ATTRIBUTE_POSITION)
        else {
            panic!("hexagon mesh must contain positions");
        };
        assert_eq!(vertices.len(), 6);
        let radius = HEX_RADIUS - HEX_GAP / 2.0;
        for index in 0..6 {
            let vertex = Vec3::from_array(vertices[index]);
            let next = Vec3::from_array(vertices[(index + 1) % 6]);
            assert!((vertex.length() - radius).abs() < 1e-5);
            assert!((vertex.distance(next) - radius).abs() < 1e-5);
        }
    }

    #[test]
    fn camera_contains_centered_board_in_landscape_and_portrait() {
        let mut app = App::new();
        app.add_systems(Startup, setup_camera);
        app.update();
        let world = app.world_mut();
        let (projection, camera_transform) = world
            .query::<(&Projection, &Transform)>()
            .single(world)
            .unwrap();
        let Projection::Orthographic(projection) = projection else {
            panic!("board camera must be orthographic");
        };
        let size = board_size();
        let mut min = Vec3::splat(f32::INFINITY);
        let mut max = Vec3::splat(f32::NEG_INFINITY);
        for row in 0..BOARD_ROWS {
            for column in 0..BOARD_COLUMNS {
                let center = HexCell { column, row }.center();
                let half_tile = Vec3::new(3.0_f32.sqrt() * HEX_RADIUS / 2.0, 0.0, HEX_RADIUS);
                min = min.min(center - half_tile);
                max = max.max(center + half_tile);
            }
        }
        assert!((min + max).abs_diff_eq(Vec3::ZERO, 1e-5));
        assert!((max - min).abs_diff_eq(Vec3::new(size.x, 0.0, size.y), 1e-5));
        assert!(camera_transform.forward().abs_diff_eq(Vec3::NEG_Y, 1e-5));
        assert!(camera_transform.up().abs_diff_eq(Vec3::NEG_Z, 1e-5));
        for (width, height) in [(1280.0, 720.0), (720.0, 1280.0), (800.0, 800.0)] {
            let mut projection = projection.clone();
            projection.update(width, height);
            assert!(projection.area.width() >= size.x + BOARD_MARGIN * 2.0 - 1e-5);
            assert!(projection.area.height() >= size.y + BOARD_MARGIN * 2.0 - 1e-5);
        }
    }
}
