// Time taken: 14:15, 14:18 -> Acc
struct Solution {}

impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        if edges[0][0] == edges[1][0] {
            return edges[0][0];
        } else if edges[0][0] == edges[1][1] {
            return edges[0][0];
        } else {
            return edges[0][1];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_center(vec![vec![1, 2], vec![2, 3], vec![4, 2]]),
            2
        );
        assert_eq!(
            Solution::find_center(vec![vec![1, 2], vec![5, 1], vec![1, 3], vec![1, 4]]),
            1
        );
    }
}
