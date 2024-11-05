use std::collections::HashMap;

// Time taken: 21:52, 21:58 -> Acc
struct Solution {}

impl Solution {
    pub fn best_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
        let mut map1 = HashMap::new();
        let mut map2 = HashMap::new();
        let mut is_pair = false;

        for val in ranks {
            *map1.entry(val).or_insert(0) += 1;
        }
        for val in suits {
            *map2.entry(val).or_insert(0) += 1;
        }

        for &val in map2.values() {
            if val == 5 {
                return String::from("Flush");
            }
        }

        for &val in map1.values() {
            if val >= 3 {
                return String::from("Three of a Kind");
            } else if val == 2 {
                is_pair = true;
            }
        }
        if is_pair {
            return String::from("Pair");
        }

        return String::from("High Card");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::best_hand(vec![13, 2, 3, 1, 9], vec!['a', 'a', 'a', 'a', 'a']),
            "Flush"
        );
        assert_eq!(
            Solution::best_hand(vec![4, 4, 2, 4, 4], vec!['d', 'a', 'a', 'b', 'c']),
            "Three of a Kind"
        );
        assert_eq!(
            Solution::best_hand(vec![10, 10, 2, 12, 9], vec!['a', 'b', 'c', 'a', 'd']),
            "Pair"
        );
    }
}
