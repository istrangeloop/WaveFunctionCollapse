#![allow(unused_imports)]

pub mod wave;
use crate::wave::Tile;
use crate::wave::Matrix;

fn main() {

    // ┼┤┌┬┐└┴┘├ | ─
    const CORNER_UL: Tile = Tile {
        up: 0,
        down: 1,
        left: 0,
        right: 1,
        drawing: "┌"
    };
    const CORNER_UR: Tile = Tile {
        up: 0,
        down: 1,
        left: 1,
        right: 0,
        drawing: "┐"
    };
    const CORNER_BL: Tile = Tile {
        up: 1,
        down: 0,
        left: 0,
        right: 1,
        drawing: "└"
    };
    const CORNER_BR: Tile = Tile {
        up: 1,
        down: 0,
        left: 1,
        right: 0,
        drawing: "┘"
    };
    const HORIZONTAL: Tile = Tile {
        up: 0,
        down: 0,
        left: 1,
        right: 1,
        drawing: "─"
    };
    const VERTICAL: Tile = Tile {
        up: 1,
        down: 1,
        left: 0,
        right: 0,
        drawing: "|"
    };
    const CROSS: Tile = Tile {
        up: 1,
        down: 1,
        left: 1,
        right: 1,
        drawing: "┼"
    };
    const TUP: Tile = Tile {
        up: 0,
        down: 1,
        left: 1,
        right: 1,
        drawing: "┬"
    };
    const TDOWN: Tile = Tile {
        up: 1,
        down: 0,
        left: 1,
        right: 1,
        drawing: "┴"
    };
    const TRIGHT: Tile = Tile {
        up: 1,
        down: 1,
        left: 1,
        right: 0,
        drawing: "┤"
    };
    const TLEFT: Tile = Tile {
        up: 1,
        down: 1,
        left: 0,
        right: 1,
        drawing: "├"
    };
    const EMPTY: Tile = Tile {
        up: 0,
        down: 0,
        left: 0,
        right: 0,
        drawing: " "
    };

    let options = vec![&EMPTY, &CORNER_BL, &CORNER_BR, &VERTICAL, &HORIZONTAL, &CORNER_UL, &CORNER_UR, &CROSS, &TUP, &TDOWN, &TRIGHT, &TLEFT];
    let mut maze = Matrix::new(30,100, options);
    maze.run();
    maze.draw();
}
