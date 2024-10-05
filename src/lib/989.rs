// Time taken: 14:02, 14:10, 14:12 -> Acc
struct Solution {}

impl Solution {
    pub fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result = Vec::new();
        let mut k = k;
        let mut remainder = 0;
        let mut completed = false;
        let mut idx = num.len() - 1;

        while k > 0 {
            let digit1 = num[idx];
            let digit2 = k % 10;
            let sum = (digit1 + digit2 + remainder) % 10;

            result.push(sum);
            remainder = (digit1 + digit2 + remainder) / 10;

            k /= 10;

            if idx == 0 {
                completed = true;
                break;
            }
            idx -= 1;
        }

        while k > 0 {
            let digit = k % 10;
            let sum = (digit + remainder) % 10;
            result.push(sum);
            remainder = (digit + remainder) / 10;
            k /= 10;
        }

        if !completed {
            loop {
                let sum = (num[idx] + remainder) % 10;
                result.push(sum);
                remainder = (num[idx] + remainder) / 10;

                if idx == 0 {
                    break;
                }
                idx -= 1;
            }
        }

        if remainder != 0 {
            result.push(remainder);
        }

        result.reverse();
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::add_to_array_form(vec![1, 2, 0, 0], 34),
            [1, 2, 3, 4]
        );
        assert_eq!(Solution::add_to_array_form(vec![2, 7, 4], 181), [4, 5, 5]);
        assert_eq!(
            Solution::add_to_array_form(vec![2, 1, 5], 806),
            [1, 0, 2, 1]
        );
    }
}
