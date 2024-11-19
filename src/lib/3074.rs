// Time taken: 00:01, 00:04 -> Acc
struct Solution {}

impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, mut capacity: Vec<i32>) -> i32 {
        capacity.sort_by(|a, b| b.cmp(&a));
        let mut sum = 0;

        for num in apple {
            sum += num;
        }

        let mut result = 0;

        for num in capacity {
            sum -= num;
            result += 1;
            if sum <= 0 {
                break;
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
        assert_eq!(
            Solution::minimum_boxes(vec![1, 3, 2], vec![4, 3, 1, 5, 2]),
            2
        );
        assert_eq!(Solution::minimum_boxes(vec![5, 5, 5], vec![2, 4, 2, 7]), 4);
    }
}
