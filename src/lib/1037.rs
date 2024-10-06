// Time taken: 14:07, 14:15
struct Solution {}

impl Solution {
    pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
        let x1 = points[0][0];
        let y1 = points[0][1];
        let x2 = points[1][0];
        let y2 = points[1][1];
        let x3 = points[2][0];
        let y3 = points[2][1];

        let sum = (x1 * (y2 - y3)) + (x2 * (y3 - y1)) + (x3 * (y1 - y2));
        return sum != 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::is_boomerang(vec![vec![1, 1], vec![2, 3], vec![3, 2]]),
            true
        );
        assert_eq!(
            Solution::is_boomerang(vec![vec![1, 1], vec![2, 2], vec![3, 3]]),
            false
        );
    }
}
