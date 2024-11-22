use std::collections::HashSet;

// Time taken: 23:04, 23:10 -> Wrong, 23:15 -> Acc
struct Solution {}

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result_set = HashSet::new();
        let len = nums.len();

        if len < 4 {
            return Vec::new();
        }

        nums.sort();

        let mut i = 0;
        while i < len - 3 {
            let mut j = i + 1;
            while j < len - 2 {
                let mut k = j + 1;
                let mut l = len - 1;
                while k < l {
                    let sum: i64 =
                        nums[i] as i64 + nums[j] as i64 + nums[k] as i64 + nums[l] as i64;
                    if sum == target as i64 {
                        let mut vec = vec![nums[i], nums[j], nums[k], nums[l]];
                        vec.sort();
                        result_set.insert(vec);

                        k += 1;
                        while k < l && nums[k] == nums[k - 1] {
                            k += 1;
                        }
                    } else if sum < target as i64 {
                        k += 1;
                    } else {
                        l -= 1;
                    }
                }

                j += 1;
            }

            i += 1;
        }

        let mut result = Vec::new();
        for vec in result_set {
            result.push(vec);
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
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            [[-2, -1, 1, 2], [-2, 0, 0, 2], [-1, 0, 0, 1]]
        );
        assert_eq!(Solution::four_sum(vec![2, 2, 2, 2, 2], 8), [[2, 2, 2, 2]]);
        assert_eq!(Solution::four_sum(vec![0], 0).is_empty(), true);
        assert_eq!(
            Solution::four_sum(vec![-1, 0, -5, -2, -2, -4, 0, 1, -2], -9),
            [
                [-5, -4, -1, 1],
                [-5, -4, 0, 0],
                [-5, -2, -2, 0],
                [-4, -2, -2, -1]
            ]
        );
        assert_eq!(
            Solution::four_sum(
                vec![1000000000, 1000000000, 1000000000, 1000000000],
                -294967296
            )
            .is_empty(),
            true
        );
    }
}
