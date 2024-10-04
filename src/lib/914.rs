use std::collections::{HashMap, HashSet};

// Time taken: 00:00, 00:07 -> Wrong, 00:10 -> Wrong, 00:25 -> Wrong, 00:28 -> Acc, 00:40 -> Optimized
struct Solution {}

impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut map = HashMap::new();

        for num in deck {
            *map.entry(num).or_insert(0) += 1;
        }

        let mut gcd = 0;
        for &val in map.values() {
            gcd = Self::gcd(gcd, val);
        }

        return gcd > 1;
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            return a;
        } else {
            return Self::gcd(b, a % b);
        }
    }

    // pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
    //     let mut map = std::collections::HashMap::new();

    //     for num in deck {
    //         if let Some(val) = map.get(&num) {
    //             map.insert(num, val + 1);
    //         } else {
    //             map.insert(num, 1);
    //         }
    //     }

    //     let mut curr_factors: Option<HashSet<i32>> = None;

    //     for (_, val) in map {
    //         if val == 1 {
    //             return false;
    //         }
    //         if let Some(factors) = curr_factors.as_mut() {
    //             let res = Self::factors(val);
    //             let mut matching_factors = HashSet::new();
    //             for num in res {
    //                 if factors.contains(&num) {
    //                     matching_factors.insert(num);
    //                 }
    //             }
    //             if matching_factors.is_empty() {
    //                 return false;
    //             } else {
    //                 curr_factors = Some(matching_factors);
    //             }
    //         } else {
    //             curr_factors = Some(Self::factors(val));
    //         }
    //     }

    //     return true;
    // }

    // fn factors(num: i32) -> HashSet<i32> {
    //     let mut factors = HashSet::new();
    //     let mut num = num;
    //     let mut val = 2;

    //     while num > 1 {
    //         if num % val == 0 {
    //             num = num / val;
    //             factors.insert(val);
    //         } else {
    //             val += 1;
    //         }
    //     }

    //     return factors;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::has_groups_size_x(vec![1, 2, 3, 4, 4, 3, 2, 1]),
            true
        );
        assert_eq!(
            Solution::has_groups_size_x(vec![1, 1, 1, 2, 2, 2, 3, 3]),
            false
        );
        assert_eq!(Solution::has_groups_size_x(vec![1]), false);
        assert_eq!(
            Solution::has_groups_size_x(vec![1, 1, 1, 1, 2, 2, 2, 2, 2, 2]),
            true
        );
        assert_eq!(
            Solution::has_groups_size_x(vec![
                1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3
            ]),
            false
        );
    }
}
