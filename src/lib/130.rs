// Time taken: 15:25, 15:47 -> Acc
// mod dsa;

use std::{
    cell::{Ref, RefCell},
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    rc::Rc,
};

struct Solution {}

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let mut visited = HashSet::new();
        let m = board.len();
        let n = board[0].len();

        // Top & bottom row
        for col in 0..n {
            if board[0][col] == 'O' {
                Self::bfs(0, col, &board, &mut visited);
            }
            if board[m - 1][col] == 'O' {
                Self::bfs(m - 1, col, &board, &mut visited);
            }
        }

        // Left & Right col
        for row in 0..m {
            if board[row][0] == 'O' {
                Self::bfs(row, 0, &board, &mut visited);
            }
            if board[row][n - 1] == 'O' {
                Self::bfs(row, n - 1, &board, &mut visited);
            }
        }

        let mut idx1 = 0;
        while idx1 < board.len() {
            let mut idx2 = 0;
            while idx2 < board[0].len() {
                if board[idx1][idx2] == 'O' && !visited.contains(&(idx1, idx2)) {
                    board[idx1][idx2] = 'X';
                }
                idx2 += 1;
            }
            idx1 += 1;
        }
    }

    fn bfs(
        row: usize,
        col: usize,
        board: &Vec<Vec<char>>,
        mut visited: &mut HashSet<(usize, usize)>,
    ) {
        // Base case
        if visited.contains(&(row, col)) {
            return;
        }
        visited.insert((row, col));

        // Right
        if col + 1 < board[0].len() && board[row][col + 1] == 'O' {
            Self::bfs(row, col + 1, &board, &mut visited);
        }

        // Bottom
        if row + 1 < board.len() && board[row + 1][col] == 'O' {
            Self::bfs(row + 1, col, &board, &mut visited);
        }

        // Left
        if col != 0 && board[row][col - 1] == 'O' {
            Self::bfs(row, col - 1, &board, &mut visited);
        }

        // Top
        if row != 0 && board[row - 1][col] == 'O' {
            Self::bfs(row - 1, col, &board, &mut visited);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
