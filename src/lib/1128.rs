use std::collections::HashMap;

// Time taken: 00:32, 00:35 -> Acc
struct Solution {}

impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut map = HashMap::new();
        let mut result = 0;

        for pair in dominoes {
            let mut a = pair[0];
            let mut b = pair[1];

            if a > b {
                let tmp = a;
                a = b;
                b = tmp;
            }

            if let Some(val) = map.get(&(a, b)) {
                result += val;
                map.insert((a, b), val + 1);
            } else {
                map.insert((a, b), 1);
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
            Solution::num_equiv_domino_pairs(vec![vec![1, 2], vec![2, 1], vec![3, 4], vec![5, 6]]),
            1
        );
        assert_eq!(
            Solution::num_equiv_domino_pairs(vec![
                vec![1, 2],
                vec![1, 2],
                vec![1, 1],
                vec![1, 2],
                vec![2, 2]
            ]),
            3
        );
    }
}
