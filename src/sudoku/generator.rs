use super::grid::Grid;
use super::solver::Solver;
use rand::Rng;

pub struct Generator;

impl Generator {
    pub fn generate(difficulty: Difficulty) -> Grid {
        let mut grid = Grid::new();

        Self::fill_diagonal_boxes(&mut grid);
        Solver::solve(&mut grid);

        let cells_to_remove = match difficulty {
            Difficulty::Easy => 35,
            Difficulty::Medium => 45,
            Difficulty::Hard => 55,
        };

        Self::remove_cells(&mut grid, cells_to_remove);

        for row in 0..9 {
            for col in 0..9 {
                if grid.get(row, col) != 0 {
                    grid.set_fixed(row, col, true);
                }
            }
        }
        grid
    }

    fn fill_diagonal_boxes(grid: &mut Grid) {
        let mut rng = rand::thread_rng();

        for box_start in (0..9).step_by(3) {
            let mut numbers: Vec<u8> = (1..=9).collect();
            for i in(1..numbers.len()).rev(){
                let j = rng.gen_range(0..=i);
                numbers.swap(i, j);
            }
            let mut idx = 0;
            for row in box_start..box_start + 3{
                for col in box_start..box_start + 3{
                    grid.set(row, col, numbers[idx]);
                    idx += 1;
                }
            }
        }
    }

    fn remove_cells(grid: &mut Grid, count: usize) {
        let mut rng = rand::thread_rng();
        let mut removed = 0;

        while removed < count {
            let row = rng.gen_range(0..9);
            let col = rng.gen_range(0..9);

            if grid.get(row, col) != 0 {
                grid.set(row, col, 0);
                removed += 1;
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}
