// Time taken: 20:31, 20:37 -> Acc

struct Solution {}

impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let len = bits.len();

        if bits[len - 1] == 1 {
            return false;
        }

        if len > 1 && bits[len - 2] == 0 {
            return true;
        }

        let mut idx = 0;
        while idx < len {
            if idx == len - 2 {
                return false;
            }
            if bits[idx] == 0 {
                idx += 1;
            } else {
                idx += 2;
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::is_one_bit_character(vec![1, 0, 0]), true);
        assert_eq!(Solution::is_one_bit_character(vec![1, 1, 1, 0]), false);
    }
}
