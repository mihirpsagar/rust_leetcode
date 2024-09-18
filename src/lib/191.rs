// Time taken: 13:10, 13:14 -> Acc
struct Solution {}

impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        return format!("{n:032b}")
            .chars()
            .filter(|&b| b == '1')
            .collect::<String>()
            .len() as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::hamming_weight(11), 3);
        assert_eq!(Solution::hamming_weight(128), 1);
        assert_eq!(Solution::hamming_weight(2147483645), 30);
    }
}
