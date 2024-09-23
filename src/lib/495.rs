// Time taken: 20:53, 21:00 -> Acc
struct Solution {}

impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        return time_series
            .iter()
            .zip(time_series.iter().skip(1))
            .fold(0, |acc, (x, y)| acc + std::cmp::min(y - x, duration))
            + duration;
    }
    // pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
    //     let mut result = 0;

    //     for idx in 0..time_series.len() {
    //         match time_series.get(idx + 1) {
    //             None => {
    //                 result += duration;
    //             }
    //             Some(&next) => {
    //                 if (time_series[idx] + duration) < next {
    //                     result += duration;
    //                 } else {
    //                     result += (next - time_series[idx]);
    //                 }
    //             }
    //         }
    //     }

    //     return result;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_poisoned_duration(vec![1, 4], 2), 4);
        assert_eq!(Solution::find_poisoned_duration(vec![1, 2], 2), 3);
    }
}
