// Time taken: 22:40, 22:48 -> Acc
struct Solution {}

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let mut x = points[0][0];
        let mut y = points[0][1];

        for point in points.iter().skip(1) {
            let diff_x = x.abs_diff(point[0]);
            let diff_y = y.abs_diff(point[1]);
            let min = std::cmp::min(diff_x, diff_y);

            result += min;
            result += diff_x - min;
            result += diff_y - min;

            x = point[0];
            y = point[1];
        }

        return result as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::min_time_to_visit_all_points(vec![vec![1, 1], vec![3, 4], vec![-1, 0]]),
            7
        );
        assert_eq!(
            Solution::min_time_to_visit_all_points(vec![vec![3, 2], vec![-2, 2]]),
            5
        );
    }
}
