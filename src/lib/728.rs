// Time taken: 20:48, 20:51, 20:55 -> Acc

struct Solution {}

impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut result = Vec::new();

        for num in left..=right {
            let mut is_valid = true;
            let mut tmp = num;
            while tmp > 0 {
                let digit = tmp % 10;
                if digit == 0 || num % digit != 0 {
                    is_valid = false;
                    break;
                }
                tmp /= 10;
            }
            if is_valid {
                result.push(num);
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
        assert_eq!(
            Solution::self_dividing_numbers(1, 22),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
        );
        assert_eq!(
            Solution::self_dividing_numbers(47, 85),
            vec![48, 55, 66, 77]
        );
    }
}
