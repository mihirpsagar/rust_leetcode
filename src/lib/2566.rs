// Time taken: 07:29, 07:35 -> Acc
struct Solution {}

impl Solution {
    pub fn min_max_difference(num: i32) -> i32 {
        let mut arr = Vec::new();
        let mut val = num;

        while val > 0 {
            arr.push(val % 10);
            val /= 10;
        }

        arr.reverse();

        let max = Self::get_max(&arr);
        let min = Self::get_min(&arr);

        return max - min;
    }

    pub fn get_max(arr: &Vec<i32>) -> i32 {
        let mut swap = 9;
        let mut result = 0;
        for val in arr {
            if *val < 9 {
                swap = *val;
                break;
            }
        }

        for val in arr {
            result *= 10;
            if *val == swap {
                result += 9;
            } else {
                result += *val;
            }
        }

        return result;
    }

    pub fn get_min(arr: &Vec<i32>) -> i32 {
        let mut swap = 0;
        let mut result = 0;
        for val in arr {
            if *val > 0 {
                swap = *val;
                break;
            }
        }

        for val in arr {
            result *= 10;
            if *val == swap {
                result += 0;
            } else {
                result += *val;
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
        assert_eq!(Solution::min_max_difference(11891), 99009);
        assert_eq!(Solution::min_max_difference(90), 99);
    }
}
