// Time taken: 15:17, 15:22 -> Acc
struct Solution {}

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        let mut sum = 0;
        let mut num = num;

        while num > 0 {
            sum += num % 10;
            num /= 10;
        }

        if sum > 0 && sum % 9 == 0 {
            return 9;
        } else {
            return sum % 9;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
