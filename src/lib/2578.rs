// Time taken: 07:59, 08:04 -> Acc
struct Solution {}

impl Solution {
    pub fn split_num(num: i32) -> i32 {
        let mut arr = Vec::new();
        let mut num1 = Vec::new();
        let mut num2 = Vec::new();

        let mut val = num;
        while val > 0 {
            arr.push(val % 10);
            val /= 10;
        }
        arr.sort();

        let mut left = true;
        for num in arr {
            if left {
                num1.push(num);
            } else {
                num2.push(num);
            }
            left = !left;
        }

        return Self::get_num(num1) + Self::get_num(num2);
    }

    pub fn get_num(arr: Vec<i32>) -> i32 {
        let mut result = 0;
        for digit in arr {
            result *= 10;
            result += digit;
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::split_num(4325), 59);
        assert_eq!(Solution::split_num(687), 75);
    }
}
