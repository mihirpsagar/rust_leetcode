// Time taken: 21:19, 21:24 -> Acc
struct Solution {}

impl Solution {
    pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
        let mut matrix = Vec::new();
        let m = m as usize;
        let n = n as usize;
        let mut result = 0;

        for _ in 0..m {
            matrix.push(vec![0; n]);
        }

        for mat in indices {
            let mut idx = 0;
            while idx < n {
                matrix[mat[0] as usize][idx] += 1;
                idx += 1;
            }
            idx = 0;
            while idx < m {
                matrix[idx][mat[1] as usize] += 1;
                idx += 1;
            }
        }

        for row in matrix {
            for num in row {
                if num % 2 != 0 {
                    result += 1;
                }
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::odd_cells(2, 3, vec![vec![0, 1], vec![1, 1]]), 6);
        assert_eq!(Solution::odd_cells(2, 2, vec![vec![1, 1], vec![0, 0]]), 0);
    }
}
