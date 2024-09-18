// Time taken: 15:46, 15:49 -> Wrong, 16:00 -> Acc
struct Solution {}

fn isBadVersion(version: i32) -> bool {
    if version >= 2147483647 {
        return true;
    } else {
        return false;
    }
}

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut left = 0;
        let mut right = n;

        while left < right {
            let mid = left + (right - left) / 2;
            if isBadVersion(mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        return left;
    }
}

fn main() {
    println!("Hello world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let sol = Solution {};
        // assert_eq!(Solution::first_bad_version(&sol, 5), 4);
        assert_eq!(Solution::first_bad_version(&sol, 2147483647), 2147483647);
    }
}
