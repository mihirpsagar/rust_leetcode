// Time taken: 16:55, 16:57 -> Acc
struct Solution {}

impl Solution {
    pub fn count_tested_devices(battery_percentages: Vec<i32>) -> i32 {
        let mut result = 0;
        for num in battery_percentages {
            if num > result {
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
        assert_eq!(Solution::count_tested_devices(vec![1, 1, 2, 1, 3]), 3);
        assert_eq!(Solution::count_tested_devices(vec![0, 1, 2]), 2);
    }
}
