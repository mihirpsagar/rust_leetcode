// Time taken: 12:44, 12:55 -> Acc
struct NeighborSum {
    grid: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NeighborSum {
    fn new(grid: Vec<Vec<i32>>) -> Self {
        return Self { grid: grid };
    }

    fn get_pos(&self, value: i32) -> (usize, usize) {
        let mut x = 0;
        let mut y = 0;
        let len = self.grid.len();

        let mut idx1 = 0;
        'outer: while idx1 < len {
            let mut idx2 = 0;
            while idx2 < len {
                if self.grid[idx1][idx2] == value {
                    x = idx1;
                    y = idx2;
                    break 'outer;
                }
                idx2 += 1;
            }
            idx1 += 1;
        }

        return (x, y);
    }

    fn adjacent_sum(&self, value: i32) -> i32 {
        let (x, y) = self.get_pos(value);
        let len = self.grid.len();

        let mut sum = 0;
        if x > 0 {
            sum += self.grid[x - 1][y];
        }
        if x + 1 < len {
            sum += self.grid[x + 1][y];
        }
        if y > 0 {
            sum += self.grid[x][y - 1];
        }
        if y + 1 < len {
            sum += self.grid[x][y + 1];
        }

        return sum;
    }

    fn diagonal_sum(&self, value: i32) -> i32 {
        let (x, y) = self.get_pos(value);
        let len = self.grid.len();
        let mut sum = 0;

        if x > 0 && y > 0 {
            sum += self.grid[x - 1][y - 1];
        }
        if x + 1 < len && y + 1 < len {
            sum += self.grid[x + 1][y + 1];
        }
        if x > 0 && y + 1 < len {
            sum += self.grid[x - 1][y + 1];
        }
        if x + 1 < len && y > 0 {
            sum += self.grid[x + 1][y - 1];
        }

        return sum;
    }
}

/**
 * Your NeighborSum object will be instantiated and called as such:
 * let obj = NeighborSum::new(grid);
 * let ret_1: i32 = obj.adjacent_sum(value);
 * let ret_2: i32 = obj.diagonal_sum(value);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
