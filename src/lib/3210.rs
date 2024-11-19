// Time taken: 12:10, 12:13 -> Acc
struct Solution {}

impl Solution {
    pub fn get_encrypted_string(s: String, k: i32) -> String {
        let mut result = String::new();
        let s = s.chars().collect::<Vec<char>>();
        let len = s.len();
        let mut idx = 0;
        let k = k as usize % len;

        while idx < len {
            result.push(s[(idx + k) % len]);
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
        assert_eq!(
            Solution::get_encrypted_string("dart".to_string(), 3),
            "tdar"
        );
        assert_eq!(Solution::get_encrypted_string("aaa".to_string(), 1), "aaa");
    }
}
