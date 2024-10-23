// use std::collections::HashMap;

// Time taken: 17:49, 17:54 -> Acc, 18:03 -> Optimized
struct Solution {}

impl Solution {
    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        let mut arr = vec![0; 101]; //1950 - 2050
        let start = 1950;
        let mut result = (0, 0); //(year_idx, pop)

        for log in logs {
            arr[(log[0] - start) as usize] += 1;
            arr[(log[1] - start) as usize] -= 1;
        }

        let mut sum = 0;
        for (idx, &val) in arr.iter().enumerate() {
            sum += val;
            if sum > result.1 {
                result = (idx, sum);
            }
        }

        return result.0 as i32 + start;
    }

    // pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
    //     let mut map = HashMap::new();
    //     let mut result = (0, 0); //(year, pop)

    //     for log in logs {
    //         for num in log[0]..log[1] {
    //             *map.entry(num).or_insert(0) += 1;
    //         }
    //     }

    //     for (key, value) in map {
    //         if value > result.1 {
    //             result = (key, value);
    //         } else {
    //             if value == result.1 {
    //                 if key < result.0 {
    //                     result = (key, value);
    //                 }
    //             }
    //         }
    //     }

    //     return result.0;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::maximum_population(vec![vec![1993, 1999], vec![2000, 2010]]),
            1993
        );
        assert_eq!(
            Solution::maximum_population(vec![
                vec![1950, 1961],
                vec![1960, 1971],
                vec![1970, 1981]
            ]),
            1960
        );
    }
}
