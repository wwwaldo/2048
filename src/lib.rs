
use rand::Rng;

pub struct Grid {
    pub cells: [[u32; 4]; 4],
    pub last_seen: Option<(usize, usize)>,
    pub move_count: u32,
    pub score: u32,
    pub high_score: u32,
    pub game_over: bool,
}

impl Grid {
    pub fn new() -> Self {
        let mut gg: Grid =  Grid {
            cells: [[0; 4]; 4],
            last_seen: None,
            move_count: 0,
            score: 0,
            high_score: 0,
            game_over: false,
        };

        gg.fill_cell(0, 0, 2);
        gg.fill_cell(1, 1, 2);
        gg.fill_cell(1, 2, 2);

        return gg
    }

    pub fn print(&self) {
        // clear the screen
        print!("\x1b[2J\x1b[1;1H");
        println!("rustup 2048: wasd to move (don't press other buttons pls)");
        println!("High score: {}", self.high_score);
        for (row, cells_row) in self.cells.iter().enumerate() {
            for (col, &cell) in cells_row.iter().enumerate() {
                if self.last_seen == Some((row, col)) {
                    print!("\x1b[32m| {:3} |\x1b[0m", cell);
                } else {
                    print!("| {:3} |", cell);
                }
            }
            println!();
        }
        print!("Move count: {}\n", self.move_count);
        print!("Score: {}\n", self.score);
    }

    pub fn fill_cell(&mut self, row: usize, col: usize, value: u32) {
        self.cells[row][col] = value;
    }

    pub fn get_empty_cells(&self) -> Vec<(usize, usize)> {
        let mut empty_cells = Vec::new();

        for (row, cells_row) in self.cells.iter().enumerate() {
            for (col, &cell) in cells_row.iter().enumerate() {
                if cell == 0 {
                    empty_cells.push((row, col));
                }
            }
        }

        empty_cells
    }

    pub fn pick_random_empty_cell(&self) -> Option<(usize, usize)> {
        let empty_cells = self.get_empty_cells();
        if empty_cells.is_empty() {
            None
        } else {
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..empty_cells.len());
            Some(empty_cells[index])
        }
    }

    pub fn step(&mut self) {
        // Implement the logic for a single step in the game
        // if let Some((row, col)) = self.pick_random_empty_cell() {
        //     let mut rng = rand::thread_rng();
        //     let value = if rng.gen_bool(0.9) { 2 } else { 4 }; // oh my god it knows 2048
        //     self.fill_cell(row, col, value);
        //     self.last_seen = Some((row, col));
        // } else {
        //     self.game_over = true;
        //     if self.score > self.high_score {
        //         self.high_score = self.score;
        //     }
        // }

        self.move_count += 1;

    }

    pub fn move_up(&mut self) {
        // get all nonzero vals ordered by row highest to lowest
        let mut ordered_vals: Vec<(usize, usize)> = Vec::new();
        for col in 0..4 {
            for row in 0..4 {
                if self.cells[row][col] != 0 {
                    ordered_vals.push((row, col));
                }
            }
        }
        // if ordered vals is empty return
        if ordered_vals.is_empty() {
            return;
        }

        // move the cells up
        for (_i, &(row, col)) in ordered_vals.iter().enumerate() {
            let mut current_row = row;
            while current_row > 0 {
                if self.cells[current_row - 1][col] == 0 {
                    self.cells[current_row - 1][col] = self.cells[current_row][col];
                    self.cells[current_row][col] = 0;
                } else if self.cells[current_row - 1][col] == self.cells[current_row][col] {
                    // merge the cells
                    self.cells[current_row - 1][col] *= 2;
                    self.score += self.cells[current_row - 1][col];
                    self.cells[current_row][col] = 0;
                    break;
                    
                } else {
                    break;
                }
                current_row -= 1;
            }
        }
    }

    pub fn move_down(&mut self) {
        // get all nonzero vals ordered by row lowest to highest
        let mut ordered_vals: Vec<(usize, usize)> = Vec::new();
        for col in 0..4 {
            for row in (0..4).rev() {
                if self.cells[row][col] != 0 {
                    ordered_vals.push((row, col));
                }
            }
        }
        // if ordered vals is empty return
        if ordered_vals.is_empty() {
            return;
        }

        // move the cells down
        for (_i, &(row, col)) in ordered_vals.iter().enumerate() {
            let mut current_row = row;
            while current_row < 3 {
                if self.cells[current_row + 1][col] == 0 {
                    self.cells[current_row + 1][col] = self.cells[current_row][col];
                    self.cells[current_row][col] = 0;
                } else if self.cells[current_row + 1][col] == self.cells[current_row][col] {
                    // merge the cells
                    self.cells[current_row + 1][col] *= 2;
                    self.score += self.cells[current_row + 1][col];
                    self.cells[current_row][col] = 0;
                    break;
                } else {
                    break;
                }
                current_row += 1;
            }
        }
    }

    pub fn move_left(&mut self) {
        // get all nonzero vals ordered by column highest to lowest
        let mut ordered_vals: Vec<(usize, usize)> = Vec::new();
        for row in 0..4 {
            for col in 0..4 {
                if self.cells[row][col] != 0 {
                    ordered_vals.push((row, col));
                }
            }
        }
        // if ordered vals is empty return
        if ordered_vals.is_empty() {
            return;
        }

        // move the cells left
        for (_i, &(row, col)) in ordered_vals.iter().enumerate() {
            let mut current_col = col;
            while current_col > 0 {
                if self.cells[row][current_col - 1] == 0 {
                    self.cells[row][current_col - 1] = self.cells[row][current_col];
                    self.cells[row][current_col] = 0;
                } else if self.cells[row][current_col - 1] == self.cells[row][current_col] {
                    // merge the cells
                    self.cells[row][current_col - 1] *= 2;
                    self.score += self.cells[row][current_col - 1];
                    self.cells[row][current_col] = 0;
                    break;
                } else {
                    break;
                }
                current_col -= 1;
            }
        }
    }

    pub fn move_right(&mut self) {
        // get all nonzero vals ordered by column lowest to highest
        let mut ordered_vals: Vec<(usize, usize)> = Vec::new();
        for row in 0..4 {
            for col in (0..4).rev() {
                if self.cells[row][col] != 0 {
                    ordered_vals.push((row, col));
                }
            }
        }
        // if ordered vals is empty return
        if ordered_vals.is_empty() {
            return;
        }

        // move the cells right
        for (_i, &(row, col)) in ordered_vals.iter().enumerate() {
            let mut current_col = col;
            while current_col < 3 {
                if self.cells[row][current_col + 1] == 0 {
                    self.cells[row][current_col + 1] = self.cells[row][current_col];
                    self.cells[row][current_col] = 0;
                } else if self.cells[row][current_col + 1] == self.cells[row][current_col] {
                    // merge the cells
                    self.cells[row][current_col + 1] *= 2;
                    self.score += self.cells[row][current_col + 1];

                    self.cells[row][current_col] = 0;
                    break;
                } else {
                    break;
                }
                current_col += 1;
            }
        }
    }

}
