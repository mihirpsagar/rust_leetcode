// Time taken: 20:45, 20:47 -> Acc
struct Solution {}

impl Solution {
    pub fn return_to_boundary_count(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut sum = 0;
        for num in nums {
            sum += num;
            if sum == 0 {
                result += 1;
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
        assert_eq!(Solution::return_to_boundary_count(vec![2, 3, -5]), 1);
        assert_eq!(Solution::return_to_boundary_count(vec![3, 2, -3, -4]), 0);
    }
}
