use super::grid::Grid;

pub struct Solver;

impl Solver {
    pub fn solve(grid: &mut Grid) -> bool {
        Self::backtrack(grid, 0, 0)
    }

    fn backtrack(grid: &mut Grid, mut row: usize, mut col: usize) -> bool {
        loop{
            if row >= 9{
                return true;
            }

            if grid.get(row, col) == 0 {
                break;
            }

            col += 1;
            if col >= 9{
                col = 0;
                row += 1;
            }
        }

        for num in 1..=9{
            if Self::is_safe(grid, row, col, num){
                grid.set(row, col, num);

                let (next_row, next_col) = if col == 8 {
                    (row + 1, 0)
                } else {
                    (row, col + 1)
                };

                if Self::backtrack(grid, next_row, next_col){
                    return true;
                }

                grid.set(row, col, 0);
            }
        }

        false
    }

    pub fn find_next_empty(grid: &Grid, start_row: usize, start_col: usize) -> Option<(usize, usize)> {
        let mut row = start_row;
        let mut col = start_col;

        loop {
             if row >= 9 {
                return None;
             }

             if grid.get(row, col) == 0 {
                return Some((row, col));
             }

             col += 1;
             if col >= 9 {
                col = 0;
                row += 1;
             }
        }
    }

    pub fn get_valid_numbers(grid: &Grid, row: usize, col: usize) -> Vec<u8> {
        let mut valid = Vec::new();
        for num in 1..=9 {
            if Self::is_safe(grid, row, col, num){
                valid.push(num);
            }
        }
        valid
    }

    pub fn is_safe_static(grid: &Grid, row: usize, col: usize, num: u8) -> bool{
        Self::is_safe(grid, row, col, num)
    }

    fn is_safe(grid: &Grid, row: usize, col: usize, num: u8) -> bool {
        for c in 0..9{
            if grid.get(row, c) == num{
                return false;
            }
        }

        for r in 0..9{
            if grid.get(r, col) == num {
                return false;
            }
        }

        let box_row = (row / 3) * 3;
        let box_col = (col / 3) * 3;
        for r in box_row..box_row + 3{
            for c in box_col..box_col + 3{
                if grid.get(r, c) == num {
                    return false;
                }
            }
        }
        true
    }
}