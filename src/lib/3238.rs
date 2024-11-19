use std::collections::HashMap;

// Time taken: 12:35, 12:42 -> Acc
struct Solution {}

impl Solution {
    pub fn winning_player_count(n: i32, pick: Vec<Vec<i32>>) -> i32 {
        let mut player = Vec::new();
        let mut result = 0;

        for _ in 0..n {
            player.push(HashMap::new());
        }

        for arr in pick {
            let idx = arr[0] as usize;
            *player[idx].entry(arr[1]).or_insert(0) += 1;
        }

        for k in 0..n {
            let map = &player[k as usize];
            'inner: for val in map.values() {
                if *val > k {
                    result += 1;
                    break 'inner;
                }
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
        assert_eq!(
            Solution::winning_player_count(
                4,
                vec![
                    vec![0, 0],
                    vec![1, 0],
                    vec![1, 0],
                    vec![2, 1],
                    vec![2, 1],
                    vec![2, 0]
                ]
            ),
            2
        );
        assert_eq!(
            Solution::winning_player_count(5, vec![vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4]]),
            0
        );
        assert_eq!(
            Solution::winning_player_count(5, vec![vec![1, 1], vec![2, 4], vec![2, 4], vec![2, 4]]),
            1
        );
    }
}
