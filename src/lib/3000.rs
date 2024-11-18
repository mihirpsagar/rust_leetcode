// Time taken: 17:33, 17:37 -> Wrong, 17:41 -> Acc
struct Solution {}

impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        let mut max = f64::MIN;
        let mut result = 0;
        let mut min_dim = i32::MIN;

        for rect in dimensions {
            if rect[0] < min_dim && rect[1] < min_dim {
                continue;
            }

            let mut val = ((rect[0] * rect[0]) + (rect[1] * rect[1])) as f64;
            val = val.sqrt();

            if val > max {
                max = val;
                result = rect[0] * rect[1];
                min_dim = std::cmp::min(rect[0], rect[1]);
            } else if val == max {
                if (rect[0] * rect[1]) > result {
                    result = rect[0] * rect[1];
                    min_dim = std::cmp::min(rect[0], rect[1]);
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
        assert_eq!(
            Solution::area_of_max_diagonal(vec![vec![9, 3], vec![8, 6]]),
            48
        );
        assert_eq!(
            Solution::area_of_max_diagonal(vec![vec![3, 4], vec![4, 3]]),
            12
        );
    }
}
