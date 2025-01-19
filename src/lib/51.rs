// Time taken: 20:06, 20:30 -> Wrong, 21:32 -> Acc
// mod dsa;

use std::{
    cell::RefCell,
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    rc::Rc,
};

struct Solution {}

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let mut result_set = HashSet::new();
        Self::solve_n_queens_rec(0, 0, n, &mut Vec::new(), &mut result_set);

        let mut result = Vec::new();
        for row in result_set {
            result.push(row);
        }
        return result;
    }

    pub fn solve_n_queens_rec(
        row: usize,
        col: usize,
        n: usize,
        mut curr: &mut Vec<String>,
        mut result_set: &mut HashSet<Vec<String>>,
    ) {
        // Base condition
        if curr.len() == n {
            result_set.insert(curr.clone());
            return;
        }
        if row >= n || col >= n {
            return;
        }
        if !Self::can_add_queen(row, col, &curr) {
            if col < n {
                Self::solve_n_queens_rec(row, col + 1, n, &mut curr, &mut result_set);
            } else {
                Self::solve_n_queens_rec(row + 1, 0, n, &mut curr, &mut result_set);
            }
            return;
        }

        // Add queen at (row, col)
        let mut new_row = Vec::new();
        for _ in 0..col {
            new_row.push('.');
        }
        new_row.push('Q');
        for _ in col + 1..n {
            new_row.push('.');
        }

        curr.push(new_row.iter().collect());
        Self::solve_n_queens_rec(row + 1, 0, n, &mut curr, &mut result_set);
        curr.pop();

        // Don't add queen at (row, col)
        if col + 1 < n {
            Self::solve_n_queens_rec(row, col + 1, n, &mut curr, &mut result_set);
        }
    }

    fn can_add_queen(row: usize, col: usize, curr: &Vec<String>) -> bool {
        if curr.is_empty() {
            return true;
        }

        // If row is already in curr, then queen is present in row. Thus, return false.
        if row < curr.len() {
            return false;
        }

        let mut arr = Vec::new();
        for row in curr {
            let mut tmp = Vec::new();
            for ch in row.chars() {
                tmp.push(ch);
            }
            arr.push(tmp);
        }

        let n = arr[0].len();

        // Previous row same column shouldn't contain queens
        let mut idx1 = row;
        let mut idx2 = col;
        while idx1 != 0 {
            idx1 -= 1;
            if arr[idx1][idx2] == 'Q' {
                return false;
            }
        }

        // Previous rows 'X' pattern top half left
        idx1 = row;
        idx2 = col;
        while idx1 != 0 && idx2 != 0 {
            idx1 -= 1;
            idx2 -= 1;
            if arr[idx1][idx2] == 'Q' {
                return false;
            }
        }

        // Previous rows 'X' pattern top half right
        idx1 = row;
        idx2 = col;
        while idx1 != 0 && idx2 + 1 < n {
            idx1 -= 1;
            idx2 += 1;
            if arr[idx1][idx2] == 'Q' {
                return false;
            }
        }

        return true;
    }

    // fn add_to_set(row: usize, col: usize, n: usize, set: &mut HashSet<(usize, usize)>) {
    //     // Add all cols
    //     let mut idx1 = row + 1;
    //     while idx1 < n {
    //         set.insert((idx1, col));
    //         idx1 += 1;
    //     }

    //     // Add all "x" pattern bottom half
    //     idx1 = row;
    //     let mut idx2 = col;
    //     while idx1 + 1 < n && idx2 != 0 {
    //         idx1 += 1;
    //         idx2 -= 1;
    //         set.insert((idx1, idx2));
    //     }
    //     idx1 = row;
    //     idx2 = col;
    //     while idx1 + 1 < n && idx2 + 1 < n {
    //         idx1 += 1;
    //         idx2 += 1;
    //         set.insert((idx1, idx2));
    //     }
    // }

    // fn remove_from_set(row: usize, col: usize, n: usize, set: &mut HashSet<(usize, usize)>) {
    //     // Remove all cols
    //     let mut idx1 = row + 1;
    //     while idx1 < n {
    //         set.remove(&(idx1, col));
    //         idx1 += 1;
    //     }

    //     // Remove all "x" pattern bottom half
    //     idx1 = row;
    //     let mut idx2 = col;
    //     while idx1 + 1 < n && idx2 != 0 {
    //         idx1 += 1;
    //         idx2 -= 1;
    //         set.remove(&(idx1, idx2));
    //     }
    //     idx1 = row;
    //     idx2 = col;
    //     while idx1 + 1 < n && idx2 + 1 < n {
    //         idx1 += 1;
    //         idx2 += 1;
    //         set.remove(&(idx1, idx2));
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::solve_n_queens(1), [["Q"]]);
        assert_eq!(
            Solution::solve_n_queens(4),
            [
                [".Q..", "...Q", "Q...", "..Q."],
                ["..Q.", "Q...", "...Q", ".Q.."]
            ]
        );
    }
}
