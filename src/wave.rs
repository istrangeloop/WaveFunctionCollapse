struct Tile {
    up: i8,
    down: i8,
    right: i8,
    left: i8,
    drawing: &'static str
}

#[derive(Clone)]
struct Space {
    // a reference to one of the fixed tiles.
    tile: &'static Tile,
    collapsed: bool,
    options: Vec<&'static Tile>
}

// -1 quer dizer um tile vazio.
impl Space {
    fn new() -> Space {
        Space {
            tile: &EMPTY,
            collapsed: false,
            options: vec![&CORNER_BL, &CORNER_BR, &VERTICAL, &HORIZONTAL, /*&CORNER_UL, &CORNER_UR, &CROSS, &TUP, &TDOWN, &TRIGHT, &TLEFT*/]
        }
    }
}

struct Matrix {
    cols: usize,
    rows: usize,
    uncollapsed: usize,
    data: Vec<Space>
}

impl Matrix {
    fn new(rows: usize, cols: usize) -> Matrix {
        Matrix {
            cols: cols,
            rows: rows,
            uncollapsed: rows*cols,
            data: vec![Space::new(); rows*cols]
        }
    }

    fn biifurcate_idx(&self, cell: usize) -> (usize, usize) {
        let col = cell % self.cols;
        let row = cell / self.cols;
        (row, col)
    }
    fn uniifiicate_idx(&self, cell: (usize, usize)) -> usize {
        let index = (cell.0 * self.cols) + cell.1;
        index
    }

    fn update_window(&mut self, cell: usize) {
        let (row, col) = self.biifurcate_idx(cell);
        let center = self.data[cell].tile;
        if row != 0 {
            let idx = self.uniifiicate_idx((row-1, col));
            let up = &mut self.data[idx];
            up.options.retain(|x| x.down == center.up);
        }
        if col != 0 {
            let idx = self.uniifiicate_idx((row, col-1));
            let left = &mut self.data[idx];
            left.options.retain(|x| x.right == center.left);
        }
        if col != &self.cols - 1 {
            let idx = self.uniifiicate_idx((row,col+1));
            let right = &mut self.data[idx];
            right.options.retain(|x| x.left == center.right);
        }
        if row != &self.rows - 1 {
            let idx = self.uniifiicate_idx((row+1, col));
            let down = &mut self.data[idx];
            down.options.retain(|x| x.up == center.down);
        }
    }

    fn collapse(&mut self, idx: usize) {
        if self.data[idx].collapsed == false {
            let len = self.data[idx].options.len();
            // no options, remain an empty tile and mark collapsed anyways
            if len != 0 {
                //pass reference to tile
                self.data[idx].tile = self.data[idx].options[rand::random::<usize>() % len];
                self.update_window(idx);
            }
            self.data[idx].collapsed = true;
            self.uncollapsed -= 1;
        }
    }

    fn least_entropy(&self) -> Vec<usize> {
        let mut le = NUM_OPT;
        let mut positions = vec![];
        let mut i: usize = 0;
        while i < self.rows*self.cols {
            if !self.data[i].collapsed {
                if self.data[i].options.len() < le {
                    le = self.data[i].options.len();
                    positions = vec![];
                }
                if self.data[i].options.len() == le {
                    positions.push(i);
                }
            }
            i+=1;
        }
        positions
    }

    fn run(&mut self) {
        loop {
            let candidates = self.least_entropy();
            self.collapse(candidates[rand::random::<usize>() % candidates.len()]);
            if self.uncollapsed == 0 {
                break;
            }
        }
    }

    fn draw(&self) {
        let mut count = 0;
        while count < self.rows * self.cols {
            if count % self.cols == 0 {
                print!("\n");
            }
            print!("{}", self.data[count].tile.drawing);
            count += 1;
        }
    }
}