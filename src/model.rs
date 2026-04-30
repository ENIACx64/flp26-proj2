// ============================================================
// dungeon-scribe — DungeonMap and Position
//
// The struct definition and all method signatures are provided.
// You must implement every method body that contains `todo!()`.
//
// You may add private helper methods freely.
// Do not change any public signature.
// ============================================================

use std::collections::HashMap;

use crate::error::{ParseError, ValidationError};
use crate::tile::Tile;

/// A position in the dungeon map, represented as `(row, col)`.
///
/// Both `row` and `col` are 0-based.
pub type Position = (usize, usize);

/// A successfully parsed dungeon map.
///
/// The grid is stored as a `Vec` of rows, each row being a `Vec<Tile>`.
/// `width` is the number of columns; `height` is the number of rows.
#[derive(Debug, PartialEq)]
pub struct DungeonMap {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl DungeonMap {
    // ── Parsing ──────────────────────────────────────────────────────────────

    /// Parse a multi-line `&str` into a `DungeonMap`.
    ///
    /// Each line of the input corresponds to one row of the map.
    /// Lines are split on `'\n'`; any trailing `'\r'` is stripped first so
    /// that both Unix (`\n`) and Windows (`\r\n`) files are accepted.
    ///
    /// # Errors
    ///
    /// - `ParseError::EmptyInput` — input has no non-empty lines.
    ///   Lines that contain only whitespace also count as empty.
    /// - `ParseError::JaggedMap` — a row has a different width from row 0.
    ///   The error carries the 0-based index of the offending row, the
    ///   expected width (from row 0), and the actual width found.
    /// - `ParseError::UnknownTile` — a character is not recognised.
    ///   The error carries the character and its `(row, col)` position.
    ///
    /// # Hint
    ///
    /// The idiomatic Rust approach is to use `lines().enumerate()` and then
    /// `chars().enumerate()` with `map` and `collect::<Result<Vec<_>, _>>()`.
    /// This pattern propagates errors naturally while building the
    /// `Vec<Vec<Tile>>`.
    pub fn parse(input: &str) -> Result<DungeonMap, ParseError> {
        // If input is empty or only contains empty lines, return Err(ParseError::EmptyInput)
        if input.lines().all(|line| line.trim().is_empty()) {
            return Err(ParseError::EmptyInput);
        }
        // Initializes important variables
        let mut tiles: Vec<Vec<Tile>> = Vec::new();
        let mut expected_width: Option<usize> = None;
        // Iterates through each line
        for (row_idx, row) in input.lines().enumerate() {
            // Strips trailing '\r' if present
            let row = row.trim_end_matches('\r');
            // Initializes a new row of tiles
            let mut row_tiles: Vec<Tile> = Vec::new();
            // Parses each character in the line into a Tile, collecting into a Vec<Tile>
            for (col_idx, c) in row.chars().enumerate() {
                match Tile::from_char(c) {
                    Ok(tile) => row_tiles.push(tile),
                    Err(ParseError::UnknownTile { c, .. }) => {
                        return Err(ParseError::UnknownTile {
                            c,
                            row: row_idx,
                            col: col_idx,
                        });
                    }
                    Err(e) => return Err(e),
                }
            }
            let row_width = row_tiles.len();
            // Checks the row width and compares it to the expected width
            match expected_width {
                None => {
                    expected_width = Some(row_width);
                }
                Some(width) => {
                    if row_width != width {
                        return Err(ParseError::JaggedMap {
                            row: row_idx,
                            expected: width,
                            found: row_width,
                        });
                    }
                }
            }
            // Pushes the row into the vector of vectors
            tiles.push(row_tiles);
        }
        // Calulcates the "height" (number of vectors in the outer vector)
        let height = tiles.len();
        // Calculates the width or throws an error if the input is empty
        let width = match expected_width {
            Some(width) => width,
            None => return Err(ParseError::EmptyInput),
        };
        // Returns the parsed map
        Ok(DungeonMap {
            tiles,
            width,
            height,
        })
    }

    // ── Accessors ─────────────────────────────────────────────────────────────

    /// Returns a reference to the tile at `pos = (row, col)`.
    ///
    /// Returns `None` if the position is out of bounds.
    /// The returned reference has the same lifetime as `&self`.
    pub fn get(&self, pos: Position) -> Option<&Tile> {
        let (row, col) = pos;
        // If row is out of bounds, returns None, otherwise tries to get col -> same behavior
        self.tiles.get(row).and_then(|r| r.get(col))
    }

    /// Returns the width of the map (number of columns).
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the map (number of rows).
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns an iterator over the rows of the map.
    ///
    /// Each item is a reference to one row (`&Vec<Tile>`).
    ///
    /// Note: returning `impl Iterator` from a method requires understanding
    /// that the iterator borrows from `&self`.
    pub fn rows(&self) -> impl Iterator<Item = &Vec<Tile>> {
        self.tiles.iter()
    }

    // ── Analysis ──────────────────────────────────────────────────────────────

    /// Returns a `HashMap` mapping each `Tile` variant to its count.
    ///
    /// Only tile types that appear at least once are included as keys.
    /// Absent tile types must **not** appear with a count of `0`.
    ///
    /// # Hint
    ///
    /// `self.rows().flat_map(|row| row.iter())` gives you a flat iterator
    /// over every tile in the map.
    pub fn count_tiles(&self) -> HashMap<Tile, usize> {
        // Initializes the hash map
        let mut tile_counts: HashMap<Tile, usize> = HashMap::new();
        // Iterates through all tiles and counts them by their variant
        for tile in self.rows().flat_map(|row| row.iter()) {
            // Searches for the tile variant in the hash map and inserts it if needed
            let counter = tile_counts.entry(*tile).or_insert(0);
            // Dereferences the counter and increments it
            *counter += 1;
        }
        // Returns the hash map
        tile_counts
    }

    /// Returns all positions where `tile` appears, sorted row-first then
    /// column (i.e. in reading order).
    ///
    /// Returns an empty `Vec` if the tile does not appear in the map.
    pub fn find_all(&self, tile: Tile) -> Vec<Position> {
        let mut found_tiles = Vec::new();
        // Iterates through each tile in the map
        for (row_idx, row) in self.rows().enumerate() {
            for (col_idx, col) in row.iter().enumerate() {
                if *col == tile {
                    found_tiles.push((row_idx, col_idx));
                }
            }
        }
        // Returns the vector
        found_tiles
    }

    // ── Validation ────────────────────────────────────────────────────────────

    /// Validates the structural integrity of the map.
    ///
    /// Returns `Ok(())` if and only if all of the following hold:
    /// - Exactly one `PlayerStart` tile exists.
    /// - At least one `Exit` tile exists.
    /// - At least one `Floor` tile exists.
    ///
    /// Otherwise returns `Err(errors)` where `errors` is a `Vec` containing
    /// **every** applicable `ValidationError`.
    ///
    /// # Important
    ///
    /// Unlike most `Result`-returning functions, this one must **accumulate
    /// all errors** before returning. Do not use `?` for early return here.
    /// Build a `Vec<ValidationError>`, push to it for each failing condition,
    /// and return `Err(errors)` at the end if it is non-empty.
    pub fn validate(&self) -> Result<(), Vec<ValidationError>> {
        // Initializing all important vectors
        let mut errors: Vec<ValidationError> = Vec::new();
        let found_starts = self.find_all(Tile::PlayerStart);
        let found_exits = self.find_all(Tile::Exit);
        let found_floors = self.find_all(Tile::Floor);
        // If no start
        if found_starts.is_empty() {
            errors.push(ValidationError::MissingPlayerStart);
        }
        // If multiple starts
        if found_starts.len() > 1 {
            errors.push(ValidationError::MultiplePlayerStarts(found_starts));
        }
        // If no exit
        if found_exits.is_empty() {
            errors.push(ValidationError::NoExit);
        }
        // If no floor
        if found_floors.is_empty() {
            errors.push(ValidationError::NoFloor);
        }
        // Returns Ok if ok, otherwise returns errors
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // Add your own tests here.

    #[test]
    fn parse_empty_input_returns_error() {
        assert!(DungeonMap::parse("").is_err());
    }

    #[test]
    fn parse_single_row_map() {
        let map = DungeonMap::parse("###").unwrap();
        assert_eq!(map.width(), 3);
        assert_eq!(map.height(), 1);
    }
}
