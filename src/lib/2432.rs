// Time taken: 01:02, 01:04, 01:09 -> Acc
struct Solution {}

impl Solution {
    pub fn hardest_worker(n: i32, logs: Vec<Vec<i32>>) -> i32 {
        let mut max = (0, 0); //(id, time)
        let mut curr = 0;

        for log in logs {
            if (log[1] - curr) > max.1 {
                max = (log[0], log[1] - curr);
            } else if (log[1] - curr) == max.1 {
                if log[0] < max.0 {
                    max = (log[0], log[1] - curr);
                }
            }
            curr = log[1];
        }

        return max.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::hardest_worker(10, vec![vec![0, 3], vec![2, 5], vec![0, 9], vec![1, 15]]),
            1
        );
        assert_eq!(
            Solution::hardest_worker(26, vec![vec![1, 1], vec![3, 7], vec![2, 12], vec![7, 17]]),
            3
        );
        assert_eq!(
            Solution::hardest_worker(2, vec![vec![0, 10], vec![1, 20]]),
            0
        );
    }
}
