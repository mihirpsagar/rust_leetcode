// Time taken: 11:16, 11:26, 11:34 -> Acc

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut idx = 0;
        while idx < board.len() {
            if !Self::is_row_valid(&board, idx) {
                return false;
            }
            if !Self::is_column_valid(&board, idx) {
                return false;
            }
            idx += 1;
        }

        let mut i = 0;
        while i <= 6 {
            let mut j = 0;
            while j <= 6 {
                if !Self::is_box_valid(&board, i, j) {
                    return false;
                }
                j += 3;
            }
            i += 3;
        }

        return true;
    }

    pub fn is_row_valid(board: &Vec<Vec<char>>, idx: usize) -> bool {
        let mut arr = vec![0; 9];
        let one = '1' as u8;

        let mut idx2 = 0;
        while idx2 < board.len() {
            if board[idx][idx2].is_ascii_digit() {
                arr[(board[idx][idx2] as u8 - one) as usize] += 1;
            }
            idx2 += 1;
        }

        for val in arr {
            if val > 1 {
                return false;
            }
        }

        return true;
    }

    pub fn is_column_valid(board: &Vec<Vec<char>>, idx: usize) -> bool {
        let mut arr = vec![0; 9];
        let one = '1' as u8;

        let mut idx2 = 0;
        while idx2 < board.len() {
            if board[idx2][idx].is_ascii_digit() {
                arr[(board[idx2][idx] as u8 - one) as usize] += 1;
            }
            idx2 += 1;
        }

        for val in arr {
            if val > 1 {
                return false;
            }
        }

        return true;
    }

    pub fn is_box_valid(board: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
        let mut arr = vec![0; 9];
        let one = '1' as u8;

        let mut idx1 = i;
        while idx1 < i + 3 {
            let mut idx2 = j;
            while idx2 < j + 3 {
                if board[idx1][idx2].is_ascii_digit() {
                    arr[(board[idx1][idx2] as u8 - one) as usize] += 1;
                }
                idx2 += 1;
            }
            idx1 += 1;
        }

        for val in arr {
            if val > 1 {
                return false;
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
