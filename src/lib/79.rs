// Time taken: 22:34, 23:01, 23:14 -> Wrong, 23:16 -> Acc, 23:26 -> Optimized

use std::{
    cell::RefCell,
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    rc::Rc,
};

struct Solution {}

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let mut freq_map = HashMap::new();
        for row in board.iter() {
            for val in row {
                *freq_map.entry(*val).or_insert(0) += 1;
            }
        }

        let mut word = word.chars().collect::<Vec<char>>();
        let mut freq_map_2 = HashMap::new();
        for ch in word.iter() {
            *freq_map_2.entry(*ch).or_insert(0) += 1;
        }

        for (key, val) in freq_map_2.iter() {
            if let Some(val2) = freq_map.get(key) {
                if *val > *val2 {
                    return false;
                }
            } else {
                return false;
            }
        }

        // Reverse word if frequency of first character > frequency of last character
        if word.len() > 1 && freq_map.get(&word[0]).unwrap() > freq_map.get(&word[1]).unwrap() {
            word.reverse();
        }

        let mut idx1 = 0;
        while idx1 < board.len() {
            let mut idx2 = 0;
            while idx2 < board[0].len() {
                if board[idx1][idx2] == word[0] {
                    // if Self::dfs(&board, &word, 0, idx1, idx2, &mut HashSet::new()) {
                    //     return true;
                    // }
                    if Self::dfs(&mut board, &word, 0, idx1, idx2) {
                        return true;
                    }
                }
                idx2 += 1;
            }

            idx1 += 1;
        }

        return false;
    }

    pub fn dfs(
        mut board: &mut Vec<Vec<char>>,
        word: &Vec<char>,
        idx: usize,
        row: usize,
        col: usize,
        // mut seen: &mut HashSet<(usize, usize)>,
    ) -> bool {
        let ch = board[row][col];

        // Base conditions
        if word[idx] != ch {
            return false;
        }
        if idx + 1 == word.len() {
            return true;
        }

        // seen.insert((row, col));
        let tmp = board[row][col];
        board[row][col] = '#';
        let mut result = false;

        // Check left
        if row != 0 && board[row - 1][col] != '#' {
            result = result | Self::dfs(&mut board, &word, idx + 1, row - 1, col);
        }

        // Check right
        if !result && row + 1 < board.len() && board[row + 1][col] != '#' {
            result = result | Self::dfs(&mut board, &word, idx + 1, row + 1, col);
        }

        // Check top
        if !result && col != 0 && board[row][col - 1] != '#' {
            result = result | Self::dfs(&mut board, &word, idx + 1, row, col - 1);
        }

        // Check bottom
        if !result && col + 1 < board[0].len() && board[row][col + 1] != '#' {
            result = result | Self::dfs(&mut board, &word, idx + 1, row, col + 1);
        }

        // seen.remove(&(row, col));
        board[row][col] = tmp;

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "ABCCED".to_string()
            ),
            true
        );
        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "ABCB".to_string()
            ),
            false
        );
    }
}
