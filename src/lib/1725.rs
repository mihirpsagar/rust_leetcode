// Time taken: 23:30, 23:33 -> Acc
struct Solution {}

impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        let mut result = (0, 0); //(size, count)

        for rect in rectangles {
            let val = std::cmp::min(rect[0], rect[1]);
            if val > result.0 {
                result = (val, 1);
            } else {
                if val == result.0 {
                    result.1 += 1;
                }
            }
        }

        return result.1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::count_good_rectangles(vec![vec![5, 8], vec![3, 9], vec![5, 12], vec![16, 5]]),
            3
        );
        assert_eq!(
            Solution::count_good_rectangles(vec![vec![2, 3], vec![3, 7], vec![4, 3], vec![3, 7]]),
            3
        );
    }
}
