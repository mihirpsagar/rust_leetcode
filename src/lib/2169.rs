// Time taken: 22:26, 22:29 -> Acc
struct Solution {}

impl Solution {
    pub fn count_operations(mut num1: i32, mut num2: i32) -> i32 {
        let mut result = 0;
        loop {
            if num1 == 0 || num2 == 0 {
                break;
            }
            if num1 < num2 {
                result = result + (num2 / num1);
                num2 = num2 % num1;
            } else {
                result = result + (num1 / num2);
                num1 = num1 % num2;
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_operations(2, 3), 3);
        assert_eq!(Solution::count_operations(10, 10), 1);
    }
}
