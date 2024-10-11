use std::{cmp::Ordering, collections::BinaryHeap};

// Time taken: 21:21, 21:54 -> Acc
struct Solution {}

struct RowSoldier(usize, usize); // (idx, soldier_count)

impl Ord for RowSoldier {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.1.cmp(&other.1) {
            Ordering::Less => {
                return Ordering::Less;
            }
            Ordering::Greater => {
                return Ordering::Greater;
            }
            Ordering::Equal => {
                return self.0.cmp(&other.0);
            }
        }
    }
}

impl PartialEq for RowSoldier {
    fn eq(&self, other: &Self) -> bool {
        return self.0 == other.0 && self.1 == other.1;
    }
}

impl Eq for RowSoldier {}

impl PartialOrd for RowSoldier {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut max_heap: BinaryHeap<RowSoldier> = BinaryHeap::with_capacity(k);
        let mut idx1 = 0;

        while idx1 < mat.len() {
            let mut idx2 = 0;
            let mut soldier_count = 0;
            while idx2 < mat[idx1].len() {
                if mat[idx1][idx2] == 0 {
                    break;
                }
                soldier_count += 1;
                idx2 += 1;
            }

            if max_heap.len() < k {
                max_heap.push(RowSoldier(idx1, soldier_count));
            } else {
                if max_heap.peek().unwrap().1 > soldier_count {
                    max_heap.pop();
                    max_heap.push(RowSoldier(idx1, soldier_count));
                }
            }

            idx1 += 1;
        }

        let mut result = Vec::new();
        while let Some(val) = max_heap.pop() {
            result.push(val.0 as i32);
        }

        result.reverse();
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::k_weakest_rows(
                vec![
                    vec![1, 1, 0, 0, 0],
                    vec![1, 1, 1, 1, 0],
                    vec![1, 0, 0, 0, 0],
                    vec![1, 1, 0, 0, 0],
                    vec![1, 1, 1, 1, 1]
                ],
                3
            ),
            [2, 0, 3]
        );
        assert_eq!(
            Solution::k_weakest_rows(
                vec![
                    vec![1, 0, 0, 0],
                    vec![1, 1, 1, 1],
                    vec![1, 0, 0, 0],
                    vec![1, 0, 0, 0]
                ],
                2
            ),
            [0, 2]
        );
    }
}
