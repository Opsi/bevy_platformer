use std::collections::HashSet;

use avian2d::prelude::*;
use bevy::{platform::collections::HashMap, prelude::*};
use bevy_ecs_ldtk::prelude::*;

use crate::player::{self, Player};

const WORLD_PATH: &str = "world.ldtk";
const PIXELS_PER_METER: f32 = 16.0;
const WORLD_SCALE: f32 = 1.0 / PIXELS_PER_METER;

#[derive(Component, Default)]
struct PlayerSpawn;

#[derive(Default, Bundle, LdtkEntity)]
struct PlayerSpawnBundle {
    player_spawn: PlayerSpawn,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
struct CollisionTile;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
struct CollisionTileBundle {
    collision_tile: CollisionTile,
}

#[derive(Component)]
struct SpawnedFromLdtk;

pub fn plugin(app: &mut App) {
    app.add_plugins(LdtkPlugin)
        .insert_resource(LevelSelection::index(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: false,
            },
            int_grid_rendering: IntGridRendering::Invisible,
            ..Default::default()
        })
        .add_systems(Startup, spawn_world)
        .add_systems(Update, spawn_collision)
        .add_systems(
            PostUpdate,
            spawn_player_at_spawn_point.after(TransformSystems::Propagate),
        )
        .register_ldtk_entity_for_layer::<PlayerSpawnBundle>("Entities", "PlayerSpawn")
        .register_ldtk_int_cell_for_layer::<CollisionTileBundle>("Collision", 1);
}

fn spawn_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!(
        "Spawning LDtk world from '{}' with scale {}",
        WORLD_PATH, WORLD_SCALE
    );

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load(WORLD_PATH).into(),
        transform: Transform::from_scale(Vec3::splat(WORLD_SCALE)),
        ..Default::default()
    });
}

fn spawn_player_at_spawn_point(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    spawn_points: Query<(Entity, &GlobalTransform), With<PlayerSpawn>>,
    players: Query<Entity, With<Player>>,
) {
    if !players.is_empty() {
        debug!(
            "Skipping PlayerSpawn processing because {} player entity/entities already exist",
            players.iter().count()
        );
        return;
    }

    let spawn_count = spawn_points.iter().count();

    if spawn_count == 0 {
        debug!("No PlayerSpawn entities found");
        return;
    }

    info!("Detected {} PlayerSpawn entity/entities", spawn_count);

    for (spawn_entity, spawn_transform) in &spawn_points {
        let mut translation = spawn_transform.translation();
        translation.z = 10.0;

        info!(
            "Spawning player from PlayerSpawn entity {:?} at world translation {:?}",
            spawn_entity, translation
        );

        player::spawn_player(&mut commands, &mut meshes, &mut materials, translation);
        break;
    }
}

fn spawn_collision(
    mut commands: Commands,
    collision_query: Query<(&GridCoords, &ChildOf), Added<CollisionTile>>,
    parent_query: Query<&ChildOf, Without<CollisionTile>>,
    level_query: Query<(Entity, &LevelIid)>,
    ldtk_projects: Query<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    #[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Plate {
        left: i32,
        right: i32,
    }

    struct Rect {
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
    }

    let mut level_to_collision_locations: HashMap<Entity, HashSet<GridCoords>> = HashMap::new();

    for (&grid_coords, child_of) in &collision_query {
        if let Ok(parent_child_of) = parent_query.get(child_of.parent()) {
            level_to_collision_locations
                .entry(parent_child_of.parent())
                .or_default()
                .insert(grid_coords);
        }
    }

    if collision_query.is_empty() {
        return;
    }

    let Ok(ldtk_handle) = ldtk_projects.single() else {
        return;
    };

    let Some(ldtk_project) = ldtk_project_assets.get(ldtk_handle) else {
        return;
    };

    for (level_entity, level_iid) in &level_query {
        let Some(level_collision) = level_to_collision_locations.get(&level_entity) else {
            continue;
        };

        let level = ldtk_project
            .as_standalone()
            .get_loaded_level_by_iid(&level_iid.to_string())
            .expect("spawned level should exist in the LDtk project");

        let collision_layer = level
            .layer_instances()
            .iter()
            .find(|layer| layer.identifier == "Collision")
            .expect("spawned level should contain a Collision layer");

        let width = collision_layer.c_wid;
        let height = collision_layer.c_hei;
        let grid_size = collision_layer.grid_size as f32;

        let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

        for y in 0..height {
            let mut row_plates = Vec::new();
            let mut plate_start = None;

            for x in 0..width + 1 {
                match (plate_start, level_collision.contains(&GridCoords { x, y })) {
                    (Some(start), false) => {
                        row_plates.push(Plate {
                            left: start,
                            right: x - 1,
                        });
                        plate_start = None;
                    }
                    (None, true) => plate_start = Some(x),
                    _ => {}
                }
            }

            plate_stack.push(row_plates);
        }

        let mut rect_builder: HashMap<Plate, Rect> = HashMap::new();
        let mut previous_row = Vec::new();
        let mut collision_rects = Vec::new();

        plate_stack.push(Vec::new());

        for (y, current_row) in plate_stack.into_iter().enumerate() {
            for previous_plate in &previous_row {
                if !current_row.contains(previous_plate) {
                    if let Some(rect) = rect_builder.remove(previous_plate) {
                        collision_rects.push(rect);
                    }
                }
            }

            for plate in &current_row {
                rect_builder
                    .entry(plate.clone())
                    .and_modify(|rect| rect.top += 1)
                    .or_insert(Rect {
                        left: plate.left,
                        right: plate.right,
                        bottom: y as i32,
                        top: y as i32,
                    });
            }

            previous_row = current_row;
        }

        commands.entity(level_entity).with_children(|level| {
            for rect in collision_rects {
                let width = (rect.right - rect.left + 1) as f32 * grid_size;
                let height = (rect.top - rect.bottom + 1) as f32 * grid_size;
                let x = (rect.left + rect.right + 1) as f32 * grid_size / 2.0;
                let y = (rect.bottom + rect.top + 1) as f32 * grid_size / 2.0;

                level.spawn((
                    SpawnedFromLdtk,
                    RigidBody::Static,
                    Collider::rectangle(width, height),
                    Transform::from_xyz(x, y, 0.0),
                    GlobalTransform::default(),
                ));
            }
        });
    }
}
