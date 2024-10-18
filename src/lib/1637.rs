// Time taken: 15:29, 15:34 -> Acc
struct Solution {}

impl Solution {
    pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
        let mut x = Vec::new();
        let mut result = 0;

        for point in points {
            Self::binary_insert(&mut x, point[0]);
        }

        let mut idx = 1;
        while idx < x.len() {
            let diff = x[idx] - x[idx - 1];
            if diff > result {
                result = diff;
            }
            idx += 1;
        }

        return result;
    }

    pub fn binary_insert(arr: &mut Vec<i32>, target: i32) {
        let mut left = 0;
        let mut right = arr.len();

        while left < right {
            let mid = left + (right - left) / 2;
            if arr[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        if left < arr.len() && arr[left] == target {
            return;
        }

        arr.insert(left, target);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_width_of_vertical_area(vec![
                vec![8, 7],
                vec![9, 9],
                vec![7, 4],
                vec![9, 7]
            ]),
            1
        );
        assert_eq!(
            Solution::max_width_of_vertical_area(vec![
                vec![3, 1],
                vec![9, 0],
                vec![1, 0],
                vec![1, 4],
                vec![5, 3],
                vec![8, 8]
            ]),
            3
        );
    }
}
