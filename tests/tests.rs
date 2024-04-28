extern crate twentyfortyeight;
use twentyfortyeight::Grid;

#[test]
fn test_move_up() {
    let mut grid = Grid::new();
    grid.cells = [
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [2, 0, 0, 0],
        [4, 0, 0, 0],
    ];

    grid.move_up();

    assert_eq!(
        grid.cells,
        [
            [2, 0, 0, 0],
            [4, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
        ]
    );
}

#[test]
fn test_move_down() {
    let mut grid = Grid::new();
    grid.cells = [
        [2, 0, 0, 0],
        [4, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ];

    grid.move_down();

    assert_eq!(
        grid.cells,
        [
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [2, 0, 0, 0],
            [4, 0, 0, 0],
        ]
    );
}

#[test]
fn test_move_left() {
    let mut grid = Grid::new();
    grid.cells = [
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [2, 4, 0, 0],
        [0, 0, 0, 0],
    ];

    grid.move_left();

    assert_eq!(
        grid.cells,
        [
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [2, 4, 0, 0],
            [0, 0, 0, 0],
        ]
    );
}

#[test]
fn test_move_right() {
    let mut grid = Grid::new();
    grid.cells = [
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [2, 4, 0, 0],
        [0, 0, 0, 0],
    ];

    grid.move_right();

    assert_eq!(
        grid.cells,
        [
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 2, 4],
            [0, 0, 0, 0],
        ]
    );
}
