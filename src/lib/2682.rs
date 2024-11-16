use std::collections::HashMap;

// Time taken: 12:11, 12:15 -> Wrong, 12:23 -> Wrong, 12:25 -> Acc
struct Solution {}

impl Solution {
    pub fn circular_game_losers(n: i32, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut result = Vec::new();
        let mut idx = 1;
        let mut multiplier = 1;
        loop {
            *map.entry(idx).or_insert(0) += 1;
            if *map.get(&idx).unwrap() == 2 {
                break;
            }
            idx = idx + (multiplier * k);
            if idx > n {
                idx = idx % n;
                if idx == 0 {
                    idx = n;
                }
            }
            multiplier += 1;
        }

        for idx in 1..=n {
            if !map.contains_key(&idx) {
                result.push(idx);
            }
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::circular_game_losers(5, 2), [4, 5]);
        assert_eq!(Solution::circular_game_losers(4, 4), [2, 3, 4]);
        assert_eq!(Solution::circular_game_losers(2, 1), []);
        assert_eq!(Solution::circular_game_losers(4, 3), []);
    }
}
