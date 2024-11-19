// Time taken: 15:33, 15:47 -> Acc
struct Solution {}

impl Solution {
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        for num in nums {
            result.push(Self::get_val(num));
        }

        return result;
    }

    pub fn get_val(mut num: i32) -> i32 {
        if num == 2 {
            return -1;
        }

        let mut binary = Vec::new();
        while num > 0 {
            binary.push(num % 2);
            num /= 2;
        }

        let mut idx = 1;
        let mut changed = false;
        while idx < binary.len() {
            if binary[idx] == 0 && binary[idx - 1] == 1 {
                binary[idx - 1] = 0;
                changed = true;
                break;
            }
            idx += 1;
        }

        binary.reverse();

        if !changed {
            binary[0] = 0;
        }

        let mut result = 0;
        idx = 0;
        while idx < binary.len() {
            result = (result * 2) + binary[idx];
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
        assert_eq!(Solution::min_bitwise_array(vec![2, 3, 5, 7]), [-1, 1, 4, 3]);
        assert_eq!(Solution::min_bitwise_array(vec![11, 13, 31]), [9, 12, 15]);
    }
}
