use std::collections::HashMap;

// Time taken: 10:20, 10:32 -> Acc
struct Solution {}

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut map = HashMap::new();
        for num in 1..=n {
            let mut tmp = num;
            let mut sum = 0;
            while tmp > 0 {
                sum = sum + (tmp % 10);
                tmp /= 10;
            }
            *map.entry(sum).or_insert(0) += 1;
        }

        let mut result = (-1, -1);
        for (_, val) in map {
            if val > result.0 {
                result = (val, 1);
            } else if val == result.0 {
                result.1 += 1;
            }
        }

        return result.1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_largest_group(13), 4);
        assert_eq!(Solution::count_largest_group(2), 2);
    }
}
