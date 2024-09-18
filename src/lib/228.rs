// Time taken: 13:48, 14:10 -> Acc
struct Solution {}

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut result = Vec::new();
        let mut left = None;

        for (idx, &num) in nums.iter().enumerate() {
            if left.is_none() {
                left = Some(num);
            } else {
                if num.saturating_sub(nums[idx - 1]) > 1 {
                    let left_num = left.unwrap();
                    if nums[idx - 1] == left_num {
                        result.push(left_num.to_string());
                    } else {
                        result.push(format!("{}->{}", left_num, nums[idx - 1]));
                    }
                    left = Some(num);
                }
            }
        }

        if let Some(left_num) = left {
            if nums[nums.len() - 1] == left_num {
                result.push(left_num.to_string())
            } else {
                result.push(format!("{}->{}", left_num, nums[nums.len() - 1]));
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
            Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7]),
            vec!["0->2", "4->5", "7"]
        );
        assert_eq!(
            Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]),
            vec!["0", "2->4", "6", "8->9"]
        );
        assert_eq!(
            Solution::summary_ranges(vec![-2147483648, -2147483647, 2147483647]),
            vec!["-2147483648->-2147483647", "2147483647"]
        );
    }
}
