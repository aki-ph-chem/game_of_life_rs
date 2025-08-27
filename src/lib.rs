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
    pub fn new() -> Self {
        Self {
            alive: false,
            neighbours_count: 0,
        }
    }

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
            if cfg!(debug_assertions) {
                eprint!("total_cnt: {total_cnt} ");
            }

            let x = rnd.random_range(0..(self.row_count - 1));
            let y = rnd.random_range(0..(self.col_count - 1));

            if cfg!(debug_assertions) {
                eprint!("x,y = ({x},{y}) ");
            }

            if self.states[x as usize][y as usize].alive {
                if cfg!(debug_assertions) {
                    eprintln!("continue");
                }
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
            for j in (y - 1).max(0)..(col_count.min(y + 2)) {
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

    pub fn inc_neighbours(&mut self, x: i32, y: i32) {
        Self::update_neighbours(x, y, self.row_count, self.col_count, &mut self.states, true);
    }

    pub fn new(row_count: i32, col_count: i32, fill_factor: f32) -> Self {
        let states = vec![vec![CellState::new(); col_count as usize]; row_count as usize];

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
                    && (self.states[i as usize][j as usize].neighbours_count < 2
                        || self.states[i as usize][j as usize].neighbours_count > 3)
                {
                    self.buffer[i as usize][j as usize].swap_life();
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

pub mod render {
    use super::{config, GameOfLife};
    use raylib::ffi::GetFrameTime;
    use raylib::{self, prelude::RaylibDraw};

    struct BoardRenderInfo {
        pub start_board_pos: raylib::prelude::Vector2,
        pub line_width: i32,
        pub cell_size: raylib::prelude::Vector2,
        pub line_color: raylib::prelude::Color,
        pub box_fill_color: raylib::prelude::Color,
        pub box_bg_color: raylib::prelude::Color,
    }

    impl BoardRenderInfo {
        pub fn new(
            start_board_pos: raylib::prelude::Vector2,
            line_width: i32,
            cell_size: raylib::prelude::Vector2,
        ) -> Self {
            Self {
                start_board_pos,
                line_width,
                cell_size,
                line_color: raylib::prelude::Color::DARKBROWN,
                box_fill_color: raylib::prelude::Color::ORANGE,
                box_bg_color: raylib::prelude::Color::BROWN,
            }
        }
    }

    pub struct MainRender {
        gof: GameOfLife,
        board_info: BoardRenderInfo,
        row_count: i32,
        col_count: i32,
        update_rate: f32,
        accum_time: f32,
    }

    impl MainRender {
        pub fn new() -> Self {
            Self {
                gof: GameOfLife::new(
                    config::DEFAULT_GRID_COUNT as i32,
                    config::DEFAULT_GRID_COUNT as i32,
                    config::DEFAULT_POPULATION,
                ),
                board_info: BoardRenderInfo::new(
                    raylib::prelude::Vector2::zero(),
                    0,
                    raylib::prelude::Vector2::zero(),
                ),
                row_count: config::DEFAULT_GRID_COUNT as i32,
                col_count: config::DEFAULT_GRID_COUNT as i32,
                update_rate: config::GAME_UPDATE_RATE,
                accum_time: 0.0,
            }
        }

        pub fn init(&mut self) {
            let line_width = (config::SCREEN_WIDTH).min(config::SCREEN_HEIGHT) as f32
                * config::BOARD_DIMENSION_PERCENTAGE;

            self.board_info.line_width = line_width.floor() as i32;
            self.board_info.cell_size.x = line_width / config::DEFAULT_GRID_COUNT as f32;
            self.board_info.cell_size.y = line_width / config::DEFAULT_GRID_COUNT as f32;
            self.board_info.start_board_pos.x = (config::SCREEN_WIDTH as f32 - line_width) / 2.0;
            self.board_info.start_board_pos.y = (config::SCREEN_HEIGHT as f32 - line_width) / 2.0;
        }

        pub fn draw(&mut self) {
            let (mut raylib_handle, raylib_thread) = raylib::init()
                .size(config::SCREEN_WIDTH as i32, config::SCREEN_HEIGHT as i32)
                .title("Game of Life")
                .vsync()
                .msaa_4x()
                .build();
            raylib_handle.set_target_fps(40);

            let mut is_drawing_active = true;
            while !raylib_handle.window_should_close() {
                let mut d = raylib_handle.begin_drawing(&raylib_thread);
                d.clear_background(raylib::prelude::Color::LIGHTGRAY);

                // shutdown by ESC or q
                if d.is_key_pressed(raylib::prelude::KeyboardKey::KEY_Q)
                    || d.is_key_pressed(raylib::prelude::KeyboardKey::KEY_ESCAPE)
                {
                    break;
                }
                if d.is_key_pressed(raylib::prelude::KeyboardKey::KEY_S) {
                    is_drawing_active = !is_drawing_active;
                }
                // detect mouse left click
                if d.is_mouse_button_pressed(raylib::ffi::MouseButton::MOUSE_BUTTON_LEFT) {
                    let mouse_position = d.get_mouse_position();
                    let delta_p = mouse_position - self.board_info.start_board_pos;
                    let x = (delta_p.x / self.board_info.cell_size.x).floor() as i32;
                    let y = (delta_p.y / self.board_info.cell_size.y).floor() as i32;

                    self.gof.states[x as usize][y as usize].swap_life();
                    self.gof.inc_neighbours(x, y);

                    if cfg!(debug_assertions) {
                        eprintln!("clicked!");
                        eprintln!("mouse_position: {:?}", mouse_position);
                        eprintln!("i,j: {x},{y}");
                    }
                }

                if is_drawing_active {
                    self.accum_time += unsafe { GetFrameTime() };
                    if self.accum_time >= self.update_rate {
                        self.accum_time -= self.update_rate;
                        self.gof.update_board();
                    }
                }

                // draw fill grid
                let board = &self.gof.states;
                for i in 0..config::DEFAULT_GRID_COUNT {
                    for j in 0..config::DEFAULT_GRID_COUNT {
                        let mut cur_pos = self.board_info.start_board_pos;

                        cur_pos.x += self.board_info.cell_size.x * (i as f32);
                        cur_pos.y += self.board_info.cell_size.y * (j as f32);
                        d.draw_rectangle_v(
                            cur_pos,
                            self.board_info.cell_size,
                            if board[i][j].alive {
                                self.board_info.box_fill_color
                            } else {
                                self.board_info.box_bg_color
                            },
                        );
                    }
                }

                // draw box line
                for dim in 0..2 {
                    for j in 0..=config::DEFAULT_GRID_COUNT {
                        if dim == 0 {
                            let mut start_pos = self.board_info.start_board_pos;
                            start_pos.x += j as f32 * self.board_info.line_width as f32
                                / config::DEFAULT_GRID_COUNT as f32;
                            let mut end_pos = start_pos;
                            end_pos.y += self.board_info.line_width as f32;
                            d.draw_line_v(start_pos, end_pos, self.board_info.line_color);
                        } else {
                            let mut start_pos = self.board_info.start_board_pos;
                            start_pos.y += j as f32 * self.board_info.line_width as f32
                                / config::DEFAULT_GRID_COUNT as f32;
                            let mut end_pos = start_pos;
                            end_pos.x += self.board_info.line_width as f32;
                            d.draw_line_v(start_pos, end_pos, self.board_info.line_color);
                        }
                    }
                }
                // end drawing logic in while
            }
        }
    }
}
