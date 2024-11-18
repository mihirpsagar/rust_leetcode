// Time taken: 14:10, 14:23 -> Acc
struct Solution {}

impl Solution {
    pub fn number_of_points(mut nums: Vec<Vec<i32>>) -> i32 {
        let mut merged_interval = Vec::new();
        let mut result = 0;

        nums.sort_by(|a, b| {
            return a[0].cmp(&b[0]);
        });

        merged_interval.push(nums[0].clone());
        let mut idx = 1;
        while idx < nums.len() {
            let m_idx = merged_interval.len() - 1;
            if nums[idx][0] >= merged_interval[m_idx][0]
                && nums[idx][0] <= merged_interval[m_idx][1]
            {
                merged_interval[m_idx][1] = std::cmp::max(merged_interval[m_idx][1], nums[idx][1]);
            } else {
                merged_interval.push(nums[idx].clone());
            }

            idx += 1;
        }

        for interval in merged_interval {
            result = result + (interval[1] - interval[0] + 1);
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
            Solution::number_of_points(vec![vec![3, 6], vec![1, 5], vec![4, 7]]),
            7
        );
        assert_eq!(Solution::number_of_points(vec![vec![1, 3], vec![5, 8]]), 7);
    }
}
