// Time taken: 15:19, 15:20 -> Acc
struct Solution {}

impl Solution {
    pub fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32> {
        let mut result = Vec::new();
        let mut idx = 1;
        while idx < height.len() {
            if height[idx - 1] > threshold {
                result.push(idx as i32);
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
        assert_eq!(Solution::stable_mountains(vec![1, 2, 3, 4, 5], 2), [3, 4]);
        assert_eq!(
            Solution::stable_mountains(vec![10, 1, 10, 1, 10], 3),
            [1, 3]
        );
        assert_eq!(Solution::stable_mountains(vec![10, 1, 10, 1, 10], 10), []);
    }
}
