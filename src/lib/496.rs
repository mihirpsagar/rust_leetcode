use std::collections::HashMap;

// Time taken: 19:43, 20:03 -> Acc
struct Solution {}

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut hash_map = HashMap::new();
        let mut result = Vec::new();
        let mut stack = vec![];

        for idx in 0..nums2.len() {
            while !stack.is_empty() {
                let top_idx = stack[stack.len() - 1];
                if nums2[top_idx] < nums2[idx] {
                    hash_map.insert(nums2[top_idx], nums2[idx]);
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(idx);
        }

        for num in nums1 {
            match hash_map.get(&num) {
                None => {
                    result.push(-1);
                }
                Some(&val) => {
                    result.push(val);
                }
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]),
            vec![-1, 3, -1]
        );
        assert_eq!(
            Solution::next_greater_element(vec![2, 4], vec![1, 2, 3, 4]),
            vec![3, -1]
        );
    }
}
