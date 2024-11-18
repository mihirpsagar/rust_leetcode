// Time taken: 16:47, 16:48 -> Acc
struct Solution {}

impl Solution {
    pub fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut idx = 1;

        while idx < mountain.len() - 1 {
            if mountain[idx] > mountain[idx - 1] && mountain[idx] > mountain[idx + 1] {
                result.push(idx as i32);
                idx += 2;
            } else {
                idx += 1;
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
        assert_eq!(Solution::find_peaks(vec![2, 4, 4]), []);
        assert_eq!(Solution::find_peaks(vec![1, 4, 3, 8, 5]), [1, 3]);
    }
}
