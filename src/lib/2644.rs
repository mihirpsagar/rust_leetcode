// Time taken: 09:46, 09:49 -> Acc, 09:56 -> Optimized?
struct Solution {}

impl Solution {
    pub fn max_div_score(nums: Vec<i32>, mut divisors: Vec<i32>) -> i32 {
        divisors.sort();
        let mut valid = vec![true; divisors.len()];
        let mut idx = 0;
        let mut result: Option<(i32, i32)> = None;

        while idx < divisors.len() {
            if !valid[idx] {
                idx += 1;
                continue;
            }

            let mut count = 0;
            for num in nums.iter() {
                if *num % divisors[idx] == 0 {
                    count += 1;
                }
            }

            if let Some(val) = result {
                if count > val.1 {
                    result = Some((divisors[idx], count));
                }
            } else {
                result = Some((divisors[idx], count));
            }

            let mut idx2 = idx + 1;
            while idx2 < divisors.len() {
                if valid[idx2] && divisors[idx2] % divisors[idx] == 0 {
                    valid[idx2] = false;
                }
                idx2 += 1;
            }

            idx += 1;
        }

        return result.unwrap().0;
    }

    // pub fn max_div_score(nums: Vec<i32>, divisors: Vec<i32>) -> i32 {
    //     let mut result: Option<(i32, i32)> = None;
    //     for divisor in divisors {
    //         let mut count = 0;
    //         for num in nums.iter() {
    //             if *num % divisor == 0 {
    //                 count += 1;
    //             }
    //         }

    //         if let Some(val) = result {
    //             if count > val.1 {
    //                 result = Some((divisor, count));
    //             } else if count == val.1 && divisor < val.0 {
    //                 result = Some((divisor, count));
    //             }
    //         } else {
    //             result = Some((divisor, count));
    //         }
    //     }

    //     return result.unwrap().0;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_div_score(vec![2, 9, 15, 50], vec![5, 3, 7, 2]),
            2
        );
        assert_eq!(
            Solution::max_div_score(vec![4, 7, 9, 3, 9], vec![5, 2, 3]),
            3
        );
        assert_eq!(
            Solution::max_div_score(vec![20, 14, 21, 10], vec![10, 16, 20]),
            10
        );
    }
}
