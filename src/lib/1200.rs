use std::cmp::Ordering;

// Time taken: 23:34, 23:41 -> Acc
struct Solution {}

impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        arr.sort();
        result.push(vec![arr[0], arr[1]]);

        let mut idx = 2;
        while idx < arr.len() {
            let curr_diff = arr[idx] - arr[idx - 1];
            let prev_diff = result[0][1] - result[0][0];

            match curr_diff.cmp(&prev_diff) {
                Ordering::Equal => {
                    result.push(vec![arr[idx - 1], arr[idx]]);
                }
                Ordering::Less => {
                    result.clear();
                    result.push(vec![arr[idx - 1], arr[idx]]);
                }
                Ordering::Greater => {}
            }

            idx += 1;
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
            Solution::minimum_abs_difference(vec![4, 2, 1, 3]),
            vec![[1, 2], [2, 3], [3, 4]]
        );
        assert_eq!(
            Solution::minimum_abs_difference(vec![1, 3, 6, 10, 15]),
            vec![[1, 3]]
        );
        assert_eq!(
            Solution::minimum_abs_difference(vec![3, 8, -10, 23, 19, -4, -14, 27]),
            vec![[-14, -10], [19, 23], [23, 27]]
        );
    }
}
