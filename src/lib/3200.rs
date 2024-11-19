// Time taken: 11:53, 12:00 -> Acc
struct Solution {}

impl Solution {
    pub fn max_height_of_triangle(red: i32, blue: i32) -> i32 {
        return std::cmp::max(Self::get_val(red, blue), Self::get_val(blue, red));
    }

    pub fn get_val(val1: i32, val2: i32) -> i32 {
        let mut val1 = val1;
        let mut val2 = val2;
        let mut result = 0;
        let mut decr = 1;
        let mut is_first = true;
        loop {
            if is_first {
                val1 -= decr;
                if val1 < 0 {
                    break;
                }
            } else {
                val2 -= decr;
                if val2 < 0 {
                    break;
                }
            }
            result += 1;
            decr += 1;
            is_first = !is_first;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::max_height_of_triangle(2, 4), 3);
        assert_eq!(Solution::max_height_of_triangle(2, 1), 2);
        assert_eq!(Solution::max_height_of_triangle(1, 1), 1);
        assert_eq!(Solution::max_height_of_triangle(10, 1), 2);
    }
}
