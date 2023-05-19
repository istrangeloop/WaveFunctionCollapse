pub struct Tile {
    pub up: i8,
    pub down: i8,
    pub right: i8,
    pub left: i8,
    pub drawing: &'static str
}

#[derive(Clone)]
struct Space<'a> {
    // a reference to one of the fixed tiles.
    tile: &'a Tile,
    collapsed: bool,
    options: Vec<&'a Tile>
}

impl Space<'_> {
    fn new(options: Vec<&Tile>) -> Space {
        Space {
            tile: options[0],
            collapsed: false,
            options
        }
    }
}

pub struct Matrix<'a> {
    cols: usize,
    rows: usize,
    uncollapsed: usize,
    set_size: usize,
    data: Vec<Space<'a>>
}

impl Matrix<'_> {
    pub fn new(rows: usize, cols: usize, options: Vec<&Tile>) -> Matrix {
        Matrix {
            cols,
            rows,
            uncollapsed: rows*cols,
            set_size: options.len(),
            data: vec![Space::new(options); rows*cols]
        }
    }

    fn biifurcate_idx(&self, cell: usize) -> (usize, usize) {
        let col: usize = cell % self.cols;
        let row: usize = cell / self.cols;
        (row, col)
    }
    fn uniifiicate_idx(&self, cell: (usize, usize)) -> usize {
        let index: usize = (cell.0 * self.cols) + cell.1;
        index
    }

    fn update_window(&mut self, cell: usize) {
        let (row, col) = self.biifurcate_idx(cell);
        let center = self.data[cell].tile;
        if row != 0 {
            let idx: usize = self.uniifiicate_idx((row-1, col));
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
        let mut le = self.set_size;
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

    pub fn run(&mut self) {
        loop {
            let candidates = self.least_entropy();
            self.collapse(candidates[rand::random::<usize>() % candidates.len()]);
            if self.uncollapsed == 0 {
                break;
            }
        }
    }

    pub fn draw(&self) {
        let mut count: usize = 0;
        while count < self.rows * self.cols {
            if count % self.cols == 0 {
                print!("\n");
            }
            print!("{}", self.data[count].tile.drawing);
            count += 1;
        }
    }
}