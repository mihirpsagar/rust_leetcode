// Time taken: 22:41, 22:44 -> Acc
struct Solution {}

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut result = vec![1];
        for i in 0..(row_index as usize) {
            let mut next_vec = vec![1];
            for j in 1..result.len() {
                next_vec.push(result[j] + result[j - 1]);
            }
            next_vec.push(1);
            result = next_vec;
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::get_row(0), vec![1]);
        assert_eq!(Solution::get_row(1), vec![1, 1]);
        assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
    }
}
