// Time taken: 11:07, 11:13 -> Acc
struct Solution {}

impl Solution {
    pub fn last_visited_integers(nums: Vec<i32>) -> Vec<i32> {
        let mut seen = Vec::new();
        let mut negative_count: i32 = 0;
        let mut result = Vec::new();
        let mut idx = 0;

        while idx < nums.len() {
            if nums[idx] != -1 {
                seen.push(nums[idx]);
                negative_count = 0;
            } else {
                negative_count += 1;
                if (seen.len() as i32 - negative_count) >= 0 {
                    result.push(seen[seen.len() - negative_count as usize]);
                } else {
                    result.push(-1);
                }
            }

            idx += 1;
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
            Solution::last_visited_integers(vec![1, 2, -1, -1, -1]),
            [2, 1, -1]
        );
        assert_eq!(
            Solution::last_visited_integers(vec![1, -1, 2, -1, -1]),
            [1, 2, 1]
        );
    }
}
