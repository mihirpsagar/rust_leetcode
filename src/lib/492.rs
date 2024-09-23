// Time taken: 20:40, 20:49 -> Acc
struct Solution {}

impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let mut width = (area as f64).sqrt() as i32;
        let mut result = vec![area, 1];
        while width > 1 {
            if area % width == 0 {
                let mut length = area / width;
                if length < width {
                    let temp = length;
                    length = width;
                    width = temp;
                }

                if (length - width) < (result[0] - result[1]) {
                    result[0] = length;
                    result[1] = width;
                }
            }
            width -= 1;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::construct_rectangle(4), vec![2, 2]);
        assert_eq!(Solution::construct_rectangle(37), vec![37, 1]);
        assert_eq!(Solution::construct_rectangle(122122), vec![427, 286]);
    }
}
