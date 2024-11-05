// Time taken: 22:28, 22:35 -> Acc
struct Solution {}

impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let len = grid.len();
        let mut idx1 = 0;
        let mut result = Vec::new();

        while idx1 < len {
            if idx1 + 2 >= len {
                break;
            }

            let mut idx2 = 0;
            let mut row = Vec::new();
            while idx2 < len {
                if idx2 + 2 >= len {
                    break;
                }
                row.push(Self::get_max(&grid, idx1, idx2));
                idx2 += 1;
            }

            result.push(row);
            idx1 += 1;
        }

        return result;
    }

    pub fn get_max(arr: &Vec<Vec<i32>>, idx1: usize, idx2: usize) -> i32 {
        let mut max = arr[idx1][idx2];

        for i in 0..3 {
            for j in 0..3 {
                if arr[idx1 + i][idx2 + j] > max {
                    max = arr[idx1 + i][idx2 + j];
                }
            }
        }

        return max;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::largest_local(vec![
                vec![9, 9, 8, 1],
                vec![5, 6, 2, 6],
                vec![8, 2, 6, 4],
                vec![6, 2, 2, 2]
            ]),
            [[9, 9], [8, 6]]
        );
        assert_eq!(
            Solution::largest_local(vec![
                vec![1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1],
                vec![1, 1, 2, 1, 1],
                vec![1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1]
            ]),
            [[2, 2, 2], [2, 2, 2], [2, 2, 2]]
        );
    }
}
