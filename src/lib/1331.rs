use std::collections::HashMap;

// Time taken: 20:56, 21:00 -> Acc
struct Solution {}

impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut sort_arr = arr.clone();
        let mut map = HashMap::new();
        let mut result = Vec::new();
        sort_arr.sort();

        let mut rank = 1;
        for num in sort_arr.iter() {
            if !map.contains_key(num) {
                map.insert(num, rank);
                rank += 1;
            }
        }

        for num in arr.iter() {
            result.push(*map.get(num).unwrap());
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
            Solution::array_rank_transform(vec![40, 10, 20, 30]),
            [4, 1, 2, 3]
        );
        assert_eq!(
            Solution::array_rank_transform(vec![100, 100, 100]),
            [1, 1, 1]
        );
        assert_eq!(
            Solution::array_rank_transform(vec![37, 12, 28, 9, 100, 56, 80, 5, 12]),
            [5, 3, 4, 2, 8, 6, 7, 1, 3]
        );
    }
}
