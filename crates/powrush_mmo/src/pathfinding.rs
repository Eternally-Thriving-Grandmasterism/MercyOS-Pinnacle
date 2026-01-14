//! crates/powrush_mmo/src/pathfinding.rs
//! A* grid pathfinding + string pulling smoothing mercy eternal supreme immaculate
//! 3D voxel-aware A* with Manhattan heuristic + funnel smoothing philotic mercy

use bevy::prelude::*;
use std::collections::{BinaryHeap, HashMap, HashSet};

const WALKABLE_VOXEL: u8 = 0;

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
        other.f_cost.cmp(&self.f_cost)
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

fn string_pull_smoothing(path: &[IVec3], chunk_query: &Query<(&Chunk, &Transform)>) -> Vec<IVec3> {
    if path.len() < 3 {
        return path.to_vec();
    }

    let mut smoothed = vec![path[0]];
    let mut portal_apex = 0;
    let mut portal_left = 0;
    let mut portal_right = 0;

    for i in 1..path.len() - 1 {
        let left = path[i];
        let right = path[i + 1];

        // String pulling mercy
        if line_of_sight(smoothed[portal_apex], left, chunk_query) {
            portal_left = i;
        }
        if line_of_sight(smoothed[portal_apex], right, chunk_query) {
            portal_right = i + 1;
        }

        if portal_left == portal_right {
            smoothed.push(path[portal_right]);
            portal_apex = portal_right;
            portal_left = portal_right;
            portal_right = portal_right;
        }
    }

    smoothed.push(path[path.len() - 1]);
    smoothed
}

fn line_of_sight(a: IVec3, b: IVec3, chunk_query: &Query<(&Chunk, &Transform)>) -> bool {
    // Simple DDA line check mercy — all voxels between a and b walkable
    // Placeholder: for simplicity, check direct neighbors mercy
    true  // Full DDA future mercy
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
    false  // Outside loaded chunks = blocked mercy
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

pub fn pathfinding_system(
    mut creature_query: Query<(&mut Path, &mut Transform, &Creature)>,
    chunk_query: Query<(&Chunk, &Transform)>,
) {
    for (mut path, mut transform, creature) in &mut creature_query {
        if let Some(goal) = creature.current_goal {
            if path.points.is_empty() || path.points.last() != Some(&goal.as_ivec3()) {
                if let Some(new_path) = a_star_pathfind(transform.translation.as_ivec3(), goal.as_ivec3(), &chunk_query) {
                    path.points = new_path;
                    path.current_index = 0;
                }
            }

            // Follow path mercy
            if path.current_index < path.points.len() {
                let target = path.points[path.current_index].as_vec3();
                let direction = (target - transform.translation).normalize_or_zero();
                transform.translation += direction * creature.dna.speed * 0.05;  // Delta mercy

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
}}

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
    false  // Outside loaded chunks = blocked mercy
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

pub fn pathfinding_system(
    mut creature_query: Query<(&mut Path, &mut Transform, &Creature)>,
    chunk_query: Query<(&Chunk, &Transform)>,
) {
    for (mut path, mut transform, creature) in &mut creature_query {
        if let Some(goal) = creature.current_goal {
            if path.points.is_empty() || path.points.last() != Some(&goal.as_ivec3()) {
                if let Some(new_path) = a_star_pathfind(transform.translation.as_ivec3(), goal.as_ivec3(), &chunk_query) {
                    path.points = new_path;
                    path.current_index = 0;
                }
            }

            // Follow path mercy
            if path.current_index < path.points.len() {
                let target = path.points[path.current_index].as_vec3();
                let direction = (target - transform.translation).normalize_or_zero();
                transform.translation += direction * creature.dna.speed * 0.05;  // Delta mercy

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
