// Time taken: 21:20, 21:28 -> Wrong, 22:33 -> Wrong, 22:42 -> Acc
struct Solution {}

impl Solution {
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        let mut mat = mat;
        for _ in 0..4 {
            if Self::is_equal(&mat, &target) {
                return true;
            }
            mat = Self::rotate(mat);
        }

        return false;
    }

    pub fn rotate(arr1: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut arr2 = Vec::new();
        let n = arr1.len();
        let mut idx1 = 0;
        while idx1 < n {
            let mut idx2 = n - 1;
            let mut row = Vec::new();
            loop {
                row.push(arr1[idx2][idx1]);
                if idx2 == 0 {
                    break;
                }
                idx2 -= 1;
            }
            arr2.push(row);
            idx1 += 1;
        }

        return arr2;
    }

    pub fn is_equal(arr1: &Vec<Vec<i32>>, arr2: &Vec<Vec<i32>>) -> bool {
        let n = arr1.len();
        let mut idx1 = 0;
        while idx1 < n {
            let mut idx2 = 0;
            while idx2 < n {
                if arr1[idx1][idx2] != arr2[idx1][idx2] {
                    return false;
                }
                idx2 += 1;
            }
            idx1 += 1;
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_rotation(vec![vec![0, 1], vec![1, 0]], vec![vec![1, 0], vec![0, 1]]),
            true
        );
        assert_eq!(
            Solution::find_rotation(vec![vec![0, 1], vec![1, 1]], vec![vec![1, 0], vec![0, 1]]),
            false
        );
        assert_eq!(
            Solution::find_rotation(
                vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]],
                vec![vec![1, 1, 1], vec![0, 1, 0], vec![0, 0, 0]]
            ),
            true
        );
        assert_eq!(
            Solution::find_rotation(vec![vec![0, 0], vec![1, 1]], vec![vec![0, 1], vec![1, 0]]),
            false
        );
        assert_eq!(
            Solution::find_rotation(
                vec![vec![0, 0, 1], vec![0, 1, 0], vec![1, 0, 0]],
                vec![vec![1, 0, 0], vec![0, 0, 1], vec![0, 1, 0]]
            ),
            false
        );
    }
}
