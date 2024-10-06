// Time taken: 20:08, 20:11 -> Acc
struct Solution {}

impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut expected = heights.clone();
        let mut result = 0;
        let mut idx = 0;
        expected.sort();

        while idx < heights.len() {
            if heights[idx] != expected[idx] {
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
        assert_eq!(Solution::height_checker(vec![1, 1, 4, 2, 1, 3]), 3);
        assert_eq!(Solution::height_checker(vec![5, 1, 2, 3, 4]), 5);
        assert_eq!(Solution::height_checker(vec![1, 2, 3, 4, 5]), 0);
    }
}
