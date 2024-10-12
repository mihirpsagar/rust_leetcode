// Time taken: 14:06, 14:12 -> Acc
struct Solution {}

impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort();
        let diff = arr[1] - arr[0];

        let mut idx = 2;
        while idx < arr.len() {
            if arr[idx] - arr[idx - 1] != diff {
                return false;
            }
            idx += 1;
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::can_make_arithmetic_progression(vec![3, 5, 1]),
            true
        );
        assert_eq!(
            Solution::can_make_arithmetic_progression(vec![1, 2, 4]),
            false
        );
    }
}
