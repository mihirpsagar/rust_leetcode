// Time taken: 18:45, 18:48 -> Acc
struct Solution {}

impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        let m = m as usize;
        let n = n as usize;
        let mut result = Vec::new();

        if m * n != original.len() {
            return result;
        }

        let mut idx = 0;
        let mut row = Vec::new();
        while idx < original.len() {
            row.push(original[idx]);
            if row.len() == n {
                result.push(row);
                row = Vec::new();
            }
            idx += 1;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::construct2_d_array(vec![1, 2, 3, 4], 2, 2),
            [[1, 2], [3, 4]]
        );
        assert_eq!(
            Solution::construct2_d_array(vec![1, 2, 3], 1, 3),
            [[1, 2, 3]]
        );
        assert_eq!(
            Solution::construct2_d_array(vec![1, 2], 1, 1).is_empty(),
            true
        );
    }
}
