use std::collections::BinaryHeap;

// Time taken: 14:18, 14:32, 14:37 -> Wrong, 14:38 -> Acc
struct Solution {}

impl Solution {
    // pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
    //     let mut max_heap = BinaryHeap::new();

    //     for stone in stones {
    //         max_heap.push(stone);
    //     }

    //     while max_heap.len() >= 2 {
    //         let mut max1 = max_heap.pop().unwrap();
    //         let max2 = max_heap.pop().unwrap();
    //         max1 -= max2;

    //         if max1 != 0 {
    //             max_heap.push(max1);
    //         }
    //     }

    //     if max_heap.len() != 0 {
    //         return max_heap.pop().unwrap();
    //     } else {
    //         return 0;
    //     }
    // }

    pub fn last_stone_weight(mut stones: Vec<i32>) -> i32 {
        stones.sort();

        while stones.len() > 1 {
            let mut max1 = stones.pop().unwrap();
            let max2 = stones.pop().unwrap();
            max1 -= max2;

            if max1 != 0 {
                let idx = Self::binary_search(&stones, max1);
                stones.insert(idx, max1);
            }
        }

        if stones.is_empty() {
            return 0;
        } else {
            return stones[0];
        }
    }

    fn binary_search(arr: &Vec<i32>, target: i32) -> usize {
        let mut left = 0;
        let mut right = arr.len();

        while left < right {
            let mid = left + (right - left) / 2;
            if arr[mid] <= target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        return left;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
        assert_eq!(Solution::last_stone_weight(vec![1]), 1);
        assert_eq!(Solution::last_stone_weight(vec![2, 2]), 0);
    }
}
