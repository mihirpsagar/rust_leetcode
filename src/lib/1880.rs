// Time taken: 18:40, 18:44 -> Acc
struct Solution {}

impl Solution {
    pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        let num1 = Self::num(first_word);
        let num2 = Self::num(second_word);
        let num3 = Self::num(target_word);

        return (num1 + num2) == num3;
    }

    pub fn num(word: String) -> usize {
        let mut num = 0;
        let a_ascii = 'a' as u8;

        for ch in word.chars() {
            let digit = ch as u8 - a_ascii;
            num = num * 10 + digit as usize;
        }

        return num;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::is_sum_equal("acb".to_string(), "cba".to_string(), "cdb".to_string()),
            true
        );
        assert_eq!(
            Solution::is_sum_equal("aaa".to_string(), "a".to_string(), "aab".to_string()),
            false
        );
        assert_eq!(
            Solution::is_sum_equal("aaa".to_string(), "a".to_string(), "aaaa".to_string()),
            true
        );
    }
}
