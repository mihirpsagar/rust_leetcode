// Time taken: 00:36, 00:42 -> Acc
struct Solution {}

impl Solution {
    pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
        let mut result = 0;
        let mut arr1 = Vec::new();
        let mut arr2 = Vec::new();
        let mut num1 = start;
        let mut num2 = goal;

        while num1 > 0 {
            arr1.push(num1 % 2);
            num1 /= 2;
        }
        if arr1.len() == 0 {
            arr1.push(0);
        }

        while num2 > 0 {
            arr2.push(num2 % 2);
            num2 /= 2;
        }
        if arr2.len() == 0 {
            arr2.push(0);
        }

        let mut idx1 = 0;
        let mut idx2 = 0;

        while idx1 < arr1.len() && idx2 < arr2.len() {
            if arr1[idx1] != arr2[idx2] {
                result += 1;
            }
            idx1 += 1;
            idx2 += 1;
        }

        while idx1 < arr1.len() {
            if arr1[idx1] == 1 {
                result += 1;
            }
            idx1 += 1;
        }

        while idx2 < arr2.len() {
            if arr2[idx2] == 1 {
                result += 1;
            }
            idx2 += 1;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_bit_flips(10, 7), 3);
        assert_eq!(Solution::min_bit_flips(3, 4), 3);
    }
}
