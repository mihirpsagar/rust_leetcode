// Time taken: 13:54, 13:57 -> Acc
struct Solution {}

impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        let mut result = start;
        let mut idx = 1;
        while idx < n {
            result = result ^ (start + (2 * idx));
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
        assert_eq!(Solution::xor_operation(5, 0), 8);
        assert_eq!(Solution::xor_operation(4, 3), 8);
    }
}
