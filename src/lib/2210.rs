// Time taken: 00:23, 00:29 -> Acc
struct Solution {}

impl Solution {
    pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut arr = Vec::new();
        let mut left = 0;
        let mut idx = 1;

        arr.push(nums[0]);
        while idx < nums.len() {
            if nums[idx] != nums[left] {
                arr.push(nums[idx]);
                left = idx;
            }
            idx += 1;
        }

        let mut idx = 1;
        while idx < arr.len() - 1 {
            if arr[idx - 1] < arr[idx] && arr[idx + 1] < arr[idx] {
                result += 1;
            } else if arr[idx - 1] > arr[idx] && arr[idx + 1] > arr[idx] {
                result += 1;
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
        assert_eq!(Solution::count_hill_valley(vec![2, 4, 1, 1, 6, 5]), 3);
        assert_eq!(Solution::count_hill_valley(vec![6, 6, 5, 5, 4, 1]), 0);
    }
}
