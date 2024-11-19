// Time taken: 12:24, 12:31 -> Acc
struct Solution {}

impl Solution {
    pub fn min_changes(n: i32, k: i32) -> i32 {
        let mut result = 0;
        let bit1 = Self::get_binary(n);
        let bit2 = Self::get_binary(k);

        let mut left = 0;
        let mut right = 0;

        while left < bit1.len() && right < bit2.len() {
            if bit1[left] != bit2[right] {
                if bit2[right] == 1 {
                    return -1;
                } else {
                    result += 1;
                }
            }

            left += 1;
            right += 1;
        }

        while left < bit1.len() {
            if bit1[left] == 1 {
                result += 1;
            }
            left += 1;
        }

        if right != bit2.len() {
            return -1;
        }

        return result;
    }

    pub fn get_binary(mut num: i32) -> Vec<i32> {
        let mut result = Vec::new();
        while num > 0 {
            result.push(num % 2);
            num /= 2;
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_changes(13, 4), 2);
        assert_eq!(Solution::min_changes(21, 21), 0);
        assert_eq!(Solution::min_changes(14, 13), -1);
    }
}
