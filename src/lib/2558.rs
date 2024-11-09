use std::collections::BinaryHeap;

// Time taken: 07:01, 07:07 -> Wrong, 07:16 -> Acc
struct Solution {}

impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut max_heap = BinaryHeap::new();
        let mut result = 0;

        for gift in gifts {
            max_heap.push(gift);
        }

        for _ in 0..k {
            let val = max_heap.pop().unwrap();
            max_heap.push(Self::sqrt(val));
        }

        while let Some(val) = max_heap.pop() {
            result += val as i64;
        }

        return result;
    }

    pub fn sqrt(val: i32) -> i32 {
        if val == 1 {
            return 1;
        }

        let mut left = 1;
        let mut right = (val / 2) + 1;

        while left < right {
            let mid = left + (right - left) / 2;
            if let Some(square) = mid.checked_mul(mid) {
                if square == val {
                    return mid;
                } else if square > val {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            } else {
                right = mid;
            }
        }

        return left - 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::pick_gifts(vec![25, 64, 9, 4, 100], 4), 29);
        assert_eq!(Solution::pick_gifts(vec![1, 1, 1, 1], 4), 4);
    }
}
