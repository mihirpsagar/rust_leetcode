use std::collections::HashMap;

// Time taken: 20:01, 20:05, 20:10, 20:19 -> Acc, 20:22 -> Optimized
struct Solution {}

impl Solution {
    pub fn most_visited(n: i32, rounds: Vec<i32>) -> Vec<i32> {
        let start = rounds[0];
        let end = rounds[rounds.len() - 1];
        let mut result = Vec::new();

        if start <= end {
            result.extend(start..=end);
        } else {
            result.extend(1..=end);
            result.extend(start..=n);
        }

        return result;
    }
    // pub fn most_visited(n: i32, rounds: Vec<i32>) -> Vec<i32> {
    //     let mut map = HashMap::new();
    //     let mut idx = 0;

    //     while idx < rounds.len() - 1 {
    //         let mut idx2 = rounds[idx];

    //         'inner: loop {
    //             *map.entry(idx2).or_insert(0) += 1;
    //             idx2 = (idx2 + 1) % (n + 1);
    //             if idx2 == 0 {
    //                 idx2 += 1;
    //             }
    //             if idx2 == rounds[idx + 1] {
    //                 break 'inner;
    //             }
    //         }

    //         idx += 1;
    //     }
    //     *map.entry(rounds[rounds.len() - 1]).or_insert(0) += 1;

    //     let mut result = Vec::new();
    //     let mut max = 0;

    //     for (key, val) in map {
    //         if val > max {
    //             result.clear();
    //             result.push(key);
    //             max = val;
    //         } else if val == max {
    //             result.push(key);
    //         }
    //     }

    //     result.sort();
    //     return result;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::most_visited(4, vec![1, 3, 1, 2]), [1, 2]);
        assert_eq!(
            Solution::most_visited(2, vec![2, 1, 2, 1, 2, 1, 2, 1, 2]),
            [2]
        );
        assert_eq!(
            Solution::most_visited(7, vec![1, 3, 5, 7]),
            [1, 2, 3, 4, 5, 6, 7]
        );
    }
}
