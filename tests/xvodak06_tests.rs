use dungeon_scribe::{
    analysis::reachable_floor_size, generate_report, DungeonMap, ParseError, Tile, ValidationError,
};

#[test]
fn parses_crlf_map_and_generates_valid_report() {
    let input = "#####\r\n#@.X#\r\n#####\r\n";

    let map = DungeonMap::parse(input).unwrap();

    assert_eq!(map.width(), 5);
    assert_eq!(map.height(), 3);
    assert!(map.validate().is_ok());

    let report = generate_report(&map);

    assert!(report.contains("Dimensions: 5 x 3\n"));
    assert!(report.contains("Validation: OK\n"));
    assert!(report.contains("Reachable floor from player: 3"));
}

#[test]
fn unknown_tile_reports_exact_position() {
    let input = "#####\n#@aX#\n#####";

    let result = DungeonMap::parse(input);

    assert_eq!(
        result,
        Err(ParseError::UnknownTile {
            c: 'a',
            row: 1,
            col: 2
        })
    );
}

#[test]
fn validation_collects_multiple_errors_at_once() {
    let input = "###\n###\n###";

    let map = DungeonMap::parse(input).unwrap();
    let errors = map.validate().unwrap_err();

    assert!(errors.contains(&ValidationError::MissingPlayerStart));
    assert!(errors.contains(&ValidationError::NoExit));
    assert!(errors.contains(&ValidationError::NoFloor));
    assert_eq!(errors.len(), 3);
}

#[test]
fn reachable_floor_does_not_cross_walls() {
    let input = "#########\n#@..#..X#\n#########";

    let map = DungeonMap::parse(input).unwrap();
    let start = map.find_all(Tile::PlayerStart)[0];

    assert_eq!(reachable_floor_size(&map, start), 3);
}