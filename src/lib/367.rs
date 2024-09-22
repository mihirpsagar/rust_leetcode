use std::cmp::Ordering;

// Time taken: 10:47, 10:54 -> Wrong, 10:57 -> Acc
struct Solution {}

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut result = false;

        if num == 1 {
            return true;
        }

        let mut left = 1;
        let mut right = (num / 2) + 1;

        while left < right {
            let mid = left + ((right - left) / 2);
            let prod = mid.checked_mul(mid);
            match prod {
                None => {
                    right = mid;
                }
                Some(prod) => match prod.cmp(&num) {
                    Ordering::Equal => {
                        return true;
                    }
                    Ordering::Less => {
                        left = mid + 1;
                    }
                    Ordering::Greater => {
                        right = mid;
                    }
                },
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
        assert_eq!(Solution::is_perfect_square(16), true);
        assert_eq!(Solution::is_perfect_square(14), false);
    }
}
