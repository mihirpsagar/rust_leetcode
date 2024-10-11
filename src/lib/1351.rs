// Time taken: 22:12, 22:13 -> Acc, 23:21 -> Optimized
struct Solution {}

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;

        for row in grid {
            result += Self::binary_search(row);
        }

        return result;
    }

    fn binary_search(row: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = row.len();

        while left < right {
            let mid = left + (right - left) / 2;
            if row[mid] < 0 {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        if left == 0 {
            if row[0] < 0 {
                return row.len() as i32;
            } else {
                return 0;
            }
        } else {
            return (row.len() - left) as i32;
        }
    }

    // pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    //     let mut result = 0;

    //     for row in grid {
    //         for val in row {
    //             if val < 0 {
    //                 result += 1;
    //             }
    //         }
    //     }
    //     return result;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::count_negatives(vec![
                vec![4, 3, 2, -1],
                vec![3, 2, 1, -1],
                vec![1, 1, -1, -2],
                vec![-1, -1, -2, -3]
            ]),
            8
        );
        assert_eq!(Solution::count_negatives(vec![vec![3, 2], vec![1, 0]]), 0);
    }
}
