use core::f64;

// Time taken: 00:09, 00:18 -> Wrong, 00:21 -> Acc, 00:27 -> Optimized
struct Solution {}

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut rolling_avg = 0.0;
        let mut max_avg = 0.0;
        let k = k as usize;
        let k_f64 = k as f64;

        for idx in 0..k {
            rolling_avg += nums[idx] as f64;
        }
        max_avg = rolling_avg / k_f64;

        for idx in k..nums.len() {
            rolling_avg -= nums[idx - k] as f64;
            rolling_avg += nums[idx] as f64;
            max_avg = max_avg.max(rolling_avg / k_f64);
        }

        return max_avg;
    }

    // pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    //     let mut rolling_avg = 0.0;
    //     let mut max_avg = f64::MIN;
    //     let mut idx = 0;
    //     let mut window = k;

    //     for idx in 0..nums.len() {
    //         if window == 0 {
    //             if rolling_avg / k as f64 > max_avg {
    //                 max_avg = rolling_avg / k as f64;
    //             }
    //             rolling_avg -= nums[idx - k as usize] as f64;
    //             rolling_avg += nums[idx] as f64;
    //         } else {
    //             rolling_avg += nums[idx] as f64;
    //             window -= 1;
    //         }
    //     }

    //     if rolling_avg / k as f64 > max_avg {
    //         max_avg = rolling_avg / k as f64;
    //     }

    //     return max_avg;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4),
            12.75000
        );
        assert_eq!(Solution::find_max_average(vec![5], 1), 5.00000);
        assert_eq!(Solution::find_max_average(vec![-1], 1), -1.00000);
    }
}
