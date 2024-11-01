// Time taken: 09:54, 09:59 -> TLE, 10:05 -> Acc, 10:09 -> Optimized
struct Solution {}

impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let len = colors.len();
        let mut left = 0;
        let mut right = len - 1;

        while left < len - 1 {
            if colors[0] != colors[right] {
                return right as i32;
            }
            if colors[left] != colors[len - 1] {
                return (len - 1 - left) as i32;
            }
            left += 1;
            right -= 1;
        }

        return 0;
    }

    // pub fn max_distance(colors: Vec<i32>) -> i32 {
    //     let mut result = 0;
    //     let len = colors.len();
    //     let mut idx = len - 1;

    //     loop {
    //         if colors[0] != colors[idx] {
    //             result = idx - 0;
    //             break;
    //         }
    //         if idx == 1 {
    //             break;
    //         }
    //         idx -= 1;
    //     }

    //     let mut idx = 0;
    //     while idx < len - 1 {
    //         if colors[idx] != colors[len - 1] {
    //             result = std::cmp::max(result, (len - 1 - idx));
    //             break;
    //         }
    //         idx += 1;
    //     }

    //     return result as i32;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::max_distance(vec![1, 1, 1, 6, 1, 1, 1]), 3);
        assert_eq!(Solution::max_distance(vec![1, 8, 3, 8, 3]), 4);
        assert_eq!(Solution::max_distance(vec![0, 1]), 1);
    }
}
