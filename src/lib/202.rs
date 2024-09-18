// Time taken: 13:17, 13:25 -> Acc
struct Solution {}

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut result = true;
        let mut seen: Vec<i32> = Vec::new();
        let mut val = n;

        loop {
            let mut sum = 0;
            while val > 0 {
                let digit = val % 10;
                sum = sum + (digit * digit);
                val /= 10;
            }

            if sum == 1 {
                break;
            } else if seen.contains(&sum) {
                result = false;
                break;
            } else {
                seen.push(sum);
                val = sum;
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
        assert_eq!(Solution::is_happy(19), true);
        assert_eq!(Solution::is_happy(2), false);
    }
}
