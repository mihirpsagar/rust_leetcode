// Time taken: 12:56, 12:58 -> Acc
struct Solution {}

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let mut max = candies[0];
        let mut result = Vec::new();

        for &num in candies.iter() {
            if num > max {
                max = num;
            }
        }

        for &num in candies.iter() {
            if num + extra_candies < max {
                result.push(false);
            } else {
                result.push(true);
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
            Solution::kids_with_candies(vec![2, 3, 5, 1, 3], 3),
            [true, true, true, false, true]
        );
        assert_eq!(
            Solution::kids_with_candies(vec![4, 2, 1, 1, 2], 1),
            [true, false, false, false, false]
        );
        assert_eq!(
            Solution::kids_with_candies(vec![12, 1, 12], 10),
            [true, false, true]
        );
    }
}
