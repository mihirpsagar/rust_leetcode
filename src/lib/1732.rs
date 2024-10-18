// Time taken: 23:48, 23:50 -> Acc
struct Solution {}

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut height = 0;

        for num in gain {
            height += num;
            if height > result {
                result = height;
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
        assert_eq!(Solution::largest_altitude(vec![-5, 1, 5, 0, -7]), 1);
        assert_eq!(Solution::largest_altitude(vec![-4, -3, -2, -1, 4, 3, 2]), 0);
    }
}
