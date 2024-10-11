// Time taken: 20:28, 20:34 -> Acc
struct Solution {}

impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        let threshold = (n / 2) + 1;
        'outer: for num in 1..threshold {
            let mut val1 = num;
            let mut val2 = n - num;

            while val1 > 0 {
                if val1 % 10 == 0 {
                    continue 'outer;
                }
                val1 /= 10;
            }

            while val2 > 0 {
                if val2 % 10 == 0 {
                    continue 'outer;
                }
                val2 /= 10;
            }

            return vec![num, n - num];
        }

        return vec![];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::get_no_zero_integers(2), vec![1, 1]);
        assert_eq!(Solution::get_no_zero_integers(11), vec![2, 9]);
    }
}
