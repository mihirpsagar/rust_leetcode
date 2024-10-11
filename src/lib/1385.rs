// Time taken: 02:03, 02:10 -> Acc
struct Solution {}

impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        let mut result = 0;

        'outer: for &num1 in arr1.iter() {
            for &num2 in arr2.iter() {
                if num1.abs_diff(num2) as i32 <= d {
                    continue 'outer;
                }
            }
            result += 1;
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
            Solution::find_the_distance_value(vec![4, 5, 8], vec![10, 9, 1, 8], 2),
            2
        );
        assert_eq!(
            Solution::find_the_distance_value(vec![1, 4, 2, 3], vec![-4, -3, 6, 10, 20, 30], 3),
            2
        );
        assert_eq!(
            Solution::find_the_distance_value(vec![2, 1, 100, 3], vec![-5, -2, 10, -3, 7], 6),
            1
        );
    }
}
