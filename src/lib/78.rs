// Time taken: 21:12, 21:17 -> Acc

use std::{
    cell::RefCell,
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    rc::Rc,
};

struct Solution {}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![vec![]];
        let mut idx = 0;

        while idx < nums.len() {
            let mut new_arr_combined = Vec::new();
            let mut idx2 = 0;
            while idx2 < result.len() {
                let mut new_arr = result[idx2].clone();
                new_arr.push(nums[idx]);
                new_arr_combined.push(new_arr);

                idx2 += 1;
            }

            for arr in new_arr_combined {
                result.push(arr);
            }
            idx += 1;
        }

        return result;
    }
}

fn main() {
    println!("Hello world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
