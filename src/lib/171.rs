// Time taken: 12:43, 12:49 -> Acc
struct Solution {}

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut result = 0;

        for (idx, ch) in column_title.chars().enumerate() {
            let pow = column_title.len() - idx - 1;
            let mul = (ch as u8) - b'A' + 1;
            result += 26_i32.pow(pow as u32) * mul as i32;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::title_to_number(String::from("A")), 1);
        assert_eq!(Solution::title_to_number(String::from("AB")), 28);
        assert_eq!(Solution::title_to_number(String::from("ZY")), 701);
    }
}
