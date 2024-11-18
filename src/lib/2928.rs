// Time taken: 16:09, 16:15 -> Acc
struct Solution {}

impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i32 {
        let mut result = 0;
        for c1 in 0..=limit {
            for c2 in 0..=limit {
                let c3 = n - c1 - c2;
                if c3 >= 0 && c3 <= limit {
                    result += 1;
                }
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
        assert_eq!(Solution::distribute_candies(5, 2), 3);
        assert_eq!(Solution::distribute_candies(3, 3), 10);
    }
}
