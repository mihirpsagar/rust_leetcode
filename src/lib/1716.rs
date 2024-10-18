// Time taken: 23:20, 23:24 -> Acc
struct Solution {}

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let mut result = 0;
        for idx in 0..n {
            if idx % 7 == 0 {
                result = result + ((idx / 7) + 1);
            } else {
                result = result + (((idx / 7) + 1) + idx % 7);
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
        assert_eq!(Solution::total_money(4), 10);
        assert_eq!(Solution::total_money(10), 37);
        assert_eq!(Solution::total_money(20), 96);
    }
}
