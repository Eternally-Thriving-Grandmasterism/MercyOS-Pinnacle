//! crates/powrush_mmo/src/pathfinding.rs
//! A* grid pathfinding + string pulling smoothing + full DDA line of sight mercy eternal supreme immaculate
//! 3D voxel-aware A* with Manhattan heuristic + funnel smoothing + Amanatides & Woo visibility philotic mercy

use bevy::prelude::*;
use std::collections::{BinaryHeap, HashMap, HashSet};

const WALKABLE_VOXEL: u8 = 0;  // Air mercy

#[derive(Component)]
pub struct Path {
    pub points: Vec<Vec3>,
    pub current_index: usize,
}

#[derive(Eq, PartialEq)]
struct Node {
    pos: IVec3,
    g_cost: i32,
    h_cost: i32,
    f_cost: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f_cost.cmp(&self.f_cost)  // Min-heap mercy
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn a_star_pathfind(
    start: IVec3,
    goal: IVec3,
    chunk_query: &Query<(&Chunk, &Transform)>,
) -> Option<Vec<IVec3>> {
    let mut open_set = BinaryHeap::new();
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();
    let mut closed_set = HashSet::new();

    g_score.insert(start, 0);
    open_set.push(Node {
        pos: start,
        g_cost: 0,
        h_cost: manhattan_distance(start, goal),
        f_cost: manhattan_distance(start, goal),
    });

    while let Some(current) = open_set.pop() {
        if current.pos == goal {
            let mut path = reconstruct_path(came_from, current.pos);
            path = string_pull_smoothing(&path, chunk_query);
            return Some(path);
        }

        closed_set.insert(current.pos);

        for neighbor in get_neighbors(current.pos, chunk_query) {
            if closed_set.contains(&neighbor) {
                continue;
            }

            let tentative_g = g_score[&current.pos] + 1;

            if !g_score.contains_key(&neighbor) || tentative_g < g_score[&neighbor] {
                came_from.insert(neighbor, current.pos);
                g_score.insert(neighbor, tentative_g);
                let h = manhattan_distance(neighbor, goal);
                open_set.push(Node {
                    pos: neighbor,
                    g_cost: tentative_g,
                    h_cost: h,
                    f_cost: tentative_g + h,
                });
            }
        }
    }

    None
}

fn manhattan_distance(a: IVec3, b: IVec3) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs()
}

fn get_neighbors(pos: IVec3, chunk_query: &Query<(&Chunk, &Transform)>) -> Vec<IVec3> {
    let mut neighbors = Vec::new();
    let directions = [
        IVec3::new(1, 0, 0), IVec3::new(-1, 0, 0),
        IVec3::new(0, 1, 0), IVec3::new(0, -1, 0),
        IVec3::new(0, 0, 1), IVec3::new(0, 0, -1),
    ];

    for dir in directions.iter() {
        let neighbor = pos + *dir;
        if is_walkable(neighbor, chunk_query) {
            neighbors.push(neighbor);
        }
    }

    neighbors
}

fn is_walkable(pos: IVec3, chunk_query: &Query<(&Chunk, &Transform)>) -> bool {
    for (chunk, chunk_transform) in chunk_query {
        let local = pos - chunk_transform.translation.as_ivec3();
        if local.x >= 0 && local.x < CHUNK_SIZE as i32 &&
           local.y >= 0 && local.y < CHUNK_SIZE as i32 &&
           local.z >= 0 && local.z < CHUNK_SIZE as i32 {
            let index = ChunkShape::linearize([local.x as u32, local.y as u32, local.z as u32]) as usize;
            return chunk.voxels[index] == WALKABLE_VOXEL;
        }
    }
    false
}

fn reconstruct_path(came_from: HashMap<IVec3, IVec3>, current: IVec3) -> Vec<IVec3> {
    let mut path = vec![current];
    let mut current = current;
    while let Some(&prev) = came_from.get(&current) {
        path.push(prev);
        current = prev;
    }
    path.reverse();
    path
}

// Full DDA line of sight mercy eternal — Amanatides & Woo 3D voxel traversal
fn line_of_sight(a: IVec3, b: IVec3, chunk_query: &Query<(&Chunk, &Transform)>) -> bool {
    let mut pos = a;
    let delta = b - a;
    let step = IVec3::new(
        if delta.x == 0 { 0 } else if delta.x > 0 { 1 } else { -1 },
        if delta.y == 0 { 0 } else if delta.y > 0 { 1 } else { -1 },
        if delta.z == 0 { 0 } else if delta.z > 0 { 1 } else { -1 },
    );

    let mut t_delta = Vec3::new(
        if delta.x == 0 { f32::INFINITY } else { 1.0 / delta.x.abs() as f32 },
        if delta.y == 0 { f32::INFINITY } else { 1.0 / delta.y.abs() as f32 },
        if delta.z == 0 { f32::INFINITY } else { 1.0 / delta.z.abs() as f32 },
    );

    let mut t_max = Vec3::new(
        if step.x > 0 { (pos.x + 1 - a.x) as f32 } else if step.x < 0 { (a.x - pos.x) as f32 } else { f32::INFINITY },
        if step.y > 0 { (pos.y + 1 - a.y) as f32 } else if step.y < 0 { (a.y - pos.y) as f32 } else { f32::INFINITY },
        if step.z > 0 { (pos.z + 1 - a.z) as f32 } else if step.z < 0 { (a.z - pos.z) as f32 } else { f32::INFINITY },
    ) * t_delta;

    while pos != b {
        if !is_walkable(pos, chunk_query) {
            return false;
        }

        if t_max.x < t_max.y && t_max.x < t_max.z {
            pos.x += step.x;
            t_max.x += t_delta.x;
        } else if t_max.y < t_max.z {
            pos.y += step.y;
            t_max.y += t_delta.y;
        } else {
            pos.z += step.z;
            t_max.z += t_delta.z;
        }
    }

    true  // All voxels walkable mercy eternal
}

fn string_pull_smoothing(path: &[IVec3], chunk_query: &Query<(&Chunk, &Transform)>) -> Vec<IVec3> {
    if path.len() < 3 {
        return path.to_vec();
    }

    let mut smoothed = vec![path[0]];
    let mut i = 0;

    while i < path.len() - 1 {
        let mut j = path.len() - 1;
        while j > i {
            if line_of_sight(path[i], path[j], chunk_query) {
                break;
            }
            j -= 1;
        }
        smoothed.push(path[j]);
        i = j;
    }

    smoothed
}

pub fn pathfinding_system(
    mut creature_query: Query<(&mut Path, &mut Transform, &Creature)>,
    chunk_query: Query<(&Chunk, &Transform)>,
) {
    for (mut path, mut transform, creature) in &mut creature_query {
        if let Some(goal) = creature.current_goal {
            let goal_ivec = goal.as_ivec3();
            if path.points.is_empty() || *path.points.last().unwrap().as_ivec3() != goal_ivec {
                if let Some(new_path) = a_star_pathfind(transform.translation.as_ivec3(), goal_ivec, &chunk_query) {
                    path.points = new_path.iter().map(|p| p.as_vec3()).collect();
                    path.current_index = 0;
                }
            }

            // Follow smoothed path mercy
            if path.current_index < path.points.len() {
                let target = path.points[path.current_index];
                let direction = (target - transform.translation).normalize_or_zero();
                transform.translation += direction * creature.dna.speed * 0.05;

                if (transform.translation - target).length_squared() < 1.0 {
                    path.current_index += 1;
                }
            }
        }
    }
}

pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, pathfinding_system);
    }
}
