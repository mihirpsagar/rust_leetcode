// Time taken: 13:25, 13:30 -> Acc
struct Solution {}

impl Solution {
    pub fn generate_key(mut num1: i32, mut num2: i32, mut num3: i32) -> i32 {
        let mut arr = Vec::new();
        while num1 > 0 || num2 > 0 || num3 > 0 {
            arr.push(std::cmp::min(
                std::cmp::min(num1 % 10, num2 % 10),
                num3 % 10,
            ));

            num1 /= 10;
            num2 /= 10;
            num3 /= 10;
        }

        arr.reverse();
        let mut result = 0;
        for digit in arr {
            result = (result * 10) + digit;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::generate_key(1, 10, 1000), 0);
        assert_eq!(Solution::generate_key(987, 879, 798), 777);
        assert_eq!(Solution::generate_key(1, 2, 3), 1);
    }
}
