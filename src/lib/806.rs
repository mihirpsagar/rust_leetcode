// Time taken: 01:22, 01:27 -> Acc
struct Solution {}

impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let mut result: (i32, i32) = (0, 1);
        let a_ascii = 'a' as u8;

        for ch in s.chars() {
            let pos = (ch as u8 - a_ascii) as usize;
            if result.0 + widths[pos] > 100 {
                result.1 += 1;
                result.0 = widths[pos];
            } else {
                result.0 += widths[pos];
            }
        }

        return vec![result.1, result.0];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::number_of_lines(
                vec![
                    10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                    10, 10, 10, 10, 10, 10
                ],
                "abcdefghijklmnopqrstuvwxyz".to_string()
            ),
            vec![3, 60]
        );
        assert_eq!(
            Solution::number_of_lines(
                vec![
                    4, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                    10, 10, 10, 10, 10, 10
                ],
                "bbbcccdddaaa".to_string()
            ),
            vec![2, 4]
        );
    }
}
