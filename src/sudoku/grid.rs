#[derive(Clone, Debug)]
pub struct Grid{
    cells: [[u8; 9]; 9],
    fixed: [[bool; 9]; 9],
}

impl Grid {
    pub fn new() -> Self{
        Self{
            cells: [[0; 9]; 9],
            fixed: [[false; 9]; 9],
        }
    }

    pub fn get(&self, row: usize, col: usize) -> u8 {
        self.cells[row][col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: u8) {
        self.cells[row][col] = value;
    }

    pub fn set_user(&mut self, row: usize, col: usize, value: u8) {
    if !self.fixed[row][col] {
        self.cells[row][col] = value;
    }
}

    pub fn is_fixed(&self, row: usize, col: usize) -> bool {
        self.fixed[row][col]
    }

    pub fn set_fixed(&mut self, row: usize, col: usize, fixed: bool){
        self.fixed[row][col] = fixed;
    }

    pub fn clear_non_fixed(&mut self){
        for row in 0..9{
            for col in 0..9{
                if !self.fixed[row][col]{
                    self.cells[row][col] = 0;
                }
            }
        }
    }

    pub fn is_valid(&self) -> bool {
        for i in 0..9 {
            if !self.is_valid_row(i) || !self.is_valid_col(i) || !self.is_valid_box(i) {
                return false;
            }
        }
        true
    }

    fn is_valid_row(&self, row: usize) -> bool {
        let mut seen = [false; 9];
        for col in 0..9 {
            let val = self.cells[row][col];
            if val != 0 {
                if seen[val as usize - 1]{
                    return false;
                }
                seen[val as usize - 1] = true;
            }
        }
        true
    }

    fn is_valid_col(&self, col: usize) -> bool {
        let mut seen = [false; 9];
        for row in 0..9  {
            let val = self.cells[row][col];
            if val != 0 {
                if seen[val as usize - 1] {
                    return false;
                }
                seen[val as usize - 1] = true;
            }
        }
        true
    }

    fn is_valid_box(&self, box_idx: usize) -> bool {
        let mut seen = [false; 9];
        let start_row = (box_idx / 3) * 3;
        let start_col = (box_idx % 3) * 3;

        for row in start_row..start_row + 3{
            for col in start_col..start_col + 3 {
                let val = self.cells[row][col];
                if val != 0 {
                    if seen[val as usize - 1]{
                        return false;
                    }
                    seen[val as usize - 1] = true;
                }
            }
        }
        true
    }

    pub fn is_complete(&self) -> bool {
        for row in 0..9{
            for col in 0..9 {
                if self.cells[row][col] == 0 {
                    return false;
                }
            }
        }
        self.is_valid()
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}