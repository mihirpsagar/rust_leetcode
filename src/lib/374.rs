// Time taken: 10:57, 11:02 -> Wrong, 11:05 - Acc
struct Solution {}

unsafe fn guess(num: i32) -> i32 {
    return 2;
}

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut left = 1;
        let mut right = n;

        while left < right {
            let mid = left + ((right - left) / 2);
            let result = guess(mid);

            if result == 0 {
                return mid;
            } else if result == 1 {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        return left;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        unsafe { assert_eq!(Solution::guessNumber(3), 2) };
    }
}
