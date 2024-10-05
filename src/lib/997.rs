// Time taken: 23:17, 23:22 -> Wrong, 23:29 -> Acc
struct Solution {}

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut result = vec![(0, 0); n as usize]; // (how many it trusts, how many trusts it)

        for val in trust {
            let idx1 = val[0] as usize;
            let idx2 = val[1] as usize;
            result[idx1 - 1].0 += 1;
            result[idx2 - 1].1 += 1;
        }

        let mut idx = 0;
        while idx < result.len() {
            if result[idx].0 == 0 && result[idx].1 == n - 1 {
                return idx as i32 + 1;
            }
            idx += 1;
        }

        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_judge(2, vec![vec![1, 2]]), 2);
        assert_eq!(Solution::find_judge(3, vec![vec![1, 3], vec![2, 3]]), 3);
        assert_eq!(
            Solution::find_judge(3, vec![vec![1, 3], vec![2, 3], vec![3, 1]]),
            -1
        );
        assert_eq!(Solution::find_judge(3, vec![vec![1, 2], vec![2, 3]]), -1);
    }
}
