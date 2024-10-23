// Time taken: 22:52, 23:04 -> Wrong, 23:13 -> Acc
struct Solution {}

impl Solution {
    pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
        let mut idx = left;
        while idx <= right {
            let mut found = false;
            for range in ranges.iter() {
                if range[0] <= idx && range[1] >= idx {
                    found = true;
                    idx = range[1];
                    break;
                }
            }
            if !found {
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
            Solution::is_covered(vec![vec![1, 2], vec![3, 4], vec![5, 6]], 2, 5),
            true
        );
        assert_eq!(
            Solution::is_covered(vec![vec![1, 10], vec![10, 20]], 21, 21),
            false
        );
        assert_eq!(
            Solution::is_covered(
                vec![
                    vec![25, 42],
                    vec![7, 14],
                    vec![2, 32],
                    vec![25, 28],
                    vec![39, 49],
                    vec![1, 50],
                    vec![29, 45],
                    vec![18, 47]
                ],
                15,
                38
            ),
            true
        );
    }
}
