// Time taken: 17:33, 17:42 -> Acc
struct Solution {}

impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut result = 0;
        let mut full = num_bottles;
        let mut empty = 0;

        while full != 0 || empty >= num_exchange {
            result += full;
            empty += full;
            full = 0;
            if empty >= num_exchange {
                full = empty / num_exchange;
                empty = empty % num_exchange;
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
        assert_eq!(Solution::num_water_bottles(9, 3), 13);
        assert_eq!(Solution::num_water_bottles(15, 4), 19);
    }
}
