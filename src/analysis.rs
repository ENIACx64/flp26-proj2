// ============================================================
// dungeon-scribe — reachability analysis
//
// You must implement the function body below.
// You may add private helper functions.
// ============================================================

use crate::model::{DungeonMap, Position};
use std::collections::HashSet;

/// Returns the number of passable tiles reachable from `start` via
/// 4-directional movement (up, down, left, right), without crossing walls.
///
/// A tile is passable if `Tile::is_passable()` returns `true` for it.
/// The starting tile itself is counted if it is passable.
///
/// # Returns
///
/// - `0` if `start` is out of bounds.
/// - `0` if the tile at `start` is not passable (i.e. is a `Wall`).
/// - Otherwise, the total number of passable tiles in the connected region
///   that contains `start`, including `start` itself.
///
/// # Implementation notes
///
/// A standard BFS or DFS is the expected approach:
/// - Use a `VecDeque<Position>` (BFS) or `Vec<Position>` (DFS) as the frontier.
/// - Use a `HashSet<Position>` to track visited positions.
/// - `Position` is `(usize, usize)` which is `Copy`, so you can push positions
///   into the queue without lifetime issues.
///
/// # Warning — usize subtraction
///
/// `Position` uses `usize`. Subtracting from `0_usize` causes a panic.
/// When generating the four neighbours of a position, guard against
/// `row == 0` and `col == 0` **before** subtracting.
pub fn reachable_floor_size(map: &DungeonMap, start: Position) -> usize {
    // Gets the start tile using the position in the parameter
    let start_tile = match map.get(start) {
        Some(tile) => tile,
        None => return 0,
    };
    // Tests if the start tile is passable
    if !start_tile.is_passable() {
        return 0;
    }
    // Declares queues for Depth-First Search
    let mut frontier: Vec<Position> = Vec::new();
    let mut visited: HashSet<Position> = HashSet::new();
    // Pushes start to the beginning of both queues
    frontier.push(start);
    visited.insert(start);
    // Initiates the DFS algorithm
    while let Some((row, col)) = frontier.pop() {
        // Expands the front of the frontier queue
        let neighbors = [
            if row > 0 { Some((row - 1, col)) } else { None },
            Some((row + 1, col)),
            if col > 0 { Some((row, col - 1)) } else { None },
            Some((row, col + 1)),
        ];
        // Checks neighbors and adds them to both queues if possible
        for neighbor in neighbors {
            // Checks if position is valid
            let Some(pos) = neighbor else {
                continue;
            };
            // Checks if position has already been visited
            if visited.contains(&pos) {
                continue;
            };
            // Checks if position is in bounds
            let Some(tile) = map.get(pos) else {
                continue;
            };
            // Checks if position is passable
            if !tile.is_passable() {
                continue;
            }
            // Adds the neighbor to both queues
            frontier.push(pos);
            visited.insert(pos);
        }
    }
    // Returns count of visited tiles (reachable from the start)
    visited.len()
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::DungeonMap;

    // Add your own tests here if needed.

    #[test]
    fn wall_start_returns_zero() {
        let map = DungeonMap::parse("###\n#@#\n###").unwrap();
        // (0, 0) is a Wall
        assert_eq!(reachable_floor_size(&map, (0, 0)), 0);
    }

    #[test]
    fn out_of_bounds_start_returns_zero() {
        let map = DungeonMap::parse("###\n#@#\n###").unwrap();
        assert_eq!(reachable_floor_size(&map, (99, 99)), 0);
    }
}
