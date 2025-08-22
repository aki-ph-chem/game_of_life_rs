use rand::Rng;

pub mod config {
    pub const SCREEN_WIDTH: usize = 900;
    pub const SCREEN_HEIGHT: usize = 800;

    pub const DEFAULT_GRID_COUNT: usize = 50;
    pub const DEFAULT_POPULATION: f32 = 0.4; // [0, 1], init population

    pub const BOARD_DIMENSION_PERCENTAGE: f32 = 0.8;
    pub const GAME_UPDATE_RATE: f32 = 0.2;
}

#[derive(Debug, Clone, Copy)]
pub struct CellState {
    pub alive: bool,
    pub neighbours_count: i32,
}

impl CellState {
    pub fn swap_life(&mut self) {
        self.alive = !self.alive;
    }
}

pub struct GameOfLife {
    row_count: i32,
    col_count: i32,
    buffer: Vec<Vec<CellState>>,
    states: Vec<Vec<CellState>>,
}

impl GameOfLife {
    fn init_board(&mut self, fill_factor: f32) {
        let mut rnd = rand::rng();

        let mut total_cnt =
            (self.row_count as f32 * self.col_count as f32 * fill_factor).floor() as i32;

        while total_cnt > 0 {
            let x = rnd.random_range(0..(self.row_count - 1));
            let y = rnd.random_range(0..(self.col_count - 1));
            if self.states[x as usize][y as usize].alive {
                continue;
            }

            total_cnt -= 1;
            self.states[x as usize][y as usize].swap_life();
            Self::update_neighbours(x, y, self.row_count, self.col_count, &mut self.states, true);
        }
    }

    fn update_neighbours(
        x: i32,
        y: i32,
        row_count: i32,
        col_count: i32,
        board: &mut Vec<Vec<CellState>>,
        add_neighbour: bool,
    ) {
        for i in (x - 1).max(0)..(row_count.min(x + 2)) {
            for j in (y - 1).max(0)..(col_count.midpoint(y + 2)) {
                if i == x && j == y {
                    continue;
                }
                if add_neighbour {
                    board[i as usize][j as usize].neighbours_count += 1;
                } else {
                    board[i as usize][j as usize].neighbours_count -= 1;
                }
            }
        }
    }

    pub fn new(row_count: i32, col_count: i32, fill_factor: f32) -> Self {
        let states = vec![
            vec![
                CellState {
                    alive: true,
                    neighbours_count: 0
                };
                col_count as usize
            ];
            row_count as usize
        ];

        let mut game_of_life = Self {
            row_count,
            col_count,
            buffer: states.clone(),
            states,
        };
        game_of_life.init_board(fill_factor);

        game_of_life
    }

    pub fn update_board(&mut self) {
        self.buffer = self.states.clone();

        for i in 0..self.row_count {
            for j in 0..self.col_count {
                if self.states[i as usize][j as usize].alive
                    && self.states[i as usize][j as usize].neighbours_count > 2
                    || self.states[i as usize][j as usize].neighbours_count > 3
                {
                    Self::update_neighbours(
                        i,
                        j,
                        self.row_count,
                        self.col_count,
                        &mut self.buffer,
                        false,
                    );
                } else if !self.states[i as usize][j as usize].alive
                    && self.states[i as usize][j as usize].neighbours_count == 3
                {
                    self.buffer[i as usize][j as usize].swap_life();
                    Self::update_neighbours(
                        i,
                        j,
                        self.row_count,
                        self.col_count,
                        &mut self.buffer,
                        true,
                    );
                }
            }
        }

        self.states = self.buffer.clone();
    }
}
