// Time taken: 13:19, 13:24 -> Acc
struct Solution {}

impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        let mut result = (-1, u32::MAX); // (idx, distance)

        for (idx, point) in points.iter().enumerate() {
            if point[0] != x && point[1] != y {
                continue;
            }

            let distance = x.abs_diff(point[0]) + y.abs_diff(point[1]);
            if distance < result.1 {
                result = (idx as i32, distance);
            }
        }

        return result.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::nearest_valid_point(
                3,
                4,
                vec![vec![1, 2], vec![3, 1], vec![2, 4], vec![2, 3], vec![4, 4]]
            ),
            2
        );
        assert_eq!(Solution::nearest_valid_point(3, 4, vec![vec![3, 4]]), 0);
        assert_eq!(Solution::nearest_valid_point(3, 4, vec![vec![2, 3]]), -1);
    }
}
