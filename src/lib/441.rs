use std::cmp::Ordering;

// Time taken: 14:20, 14:28 -> Wrong, 14:46 -> Wrong, 14:51 -> Acc
struct Solution {}

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let mut left: u128 = 1;
        let mut right: u128 = n as u128;
        let n = n as u128;

        while left < right {
            let mid: u128 = (left + (right - left) / 2) as u128;

            match mid.checked_mul(mid + 1) {
                None => {
                    right = mid;
                }
                Some(val) => {
                    let val = val / 2;
                    match val.cmp(&n) {
                        Ordering::Equal => {
                            return mid as i32;
                        }
                        Ordering::Greater => {
                            right = mid;
                        }
                        Ordering::Less => {
                            left = mid + 1;
                        }
                    }
                }
            }
        }

        match left.checked_mul(left + 1) {
            None => {}
            Some(val) => {
                let val = val / 2;
                if val > n {
                    left -= 1;
                }
            }
        }

        return left as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::arrange_coins(5), 2);
        assert_eq!(Solution::arrange_coins(8), 3);
        assert_eq!(Solution::arrange_coins(6), 3);
        assert_eq!(Solution::arrange_coins(1804289383), 60070);
    }
}
