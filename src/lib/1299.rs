// Time taken: 23:28, 23:32 -> Acc
struct Solution {}

impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut result = vec![-1; arr.len()];

        if arr.len() == 1 {
            return result;
        }

        let mut largest = arr[arr.len() - 1];
        let mut idx = arr.len() - 2;

        loop {
            result[idx] = largest;
            if arr[idx] > largest {
                largest = arr[idx];
            }
            if idx == 0 {
                break;
            }
            idx -= 1;
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
            Solution::replace_elements(vec![17, 18, 5, 4, 6, 1]),
            [18, 6, 6, 6, 1, -1]
        );
        assert_eq!(Solution::replace_elements(vec![400]), [-1]);
    }
}
