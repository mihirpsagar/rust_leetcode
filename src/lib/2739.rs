// Time taken: 11:35, 11:42 -> Acc

struct Solution {}

impl Solution {
    pub fn distance_traveled(mut main_tank: i32, mut additional_tank: i32) -> i32 {
        let mut result = 0;

        while main_tank >= 5 && additional_tank >= 1 {
            result += 50;
            main_tank -= 4;
            additional_tank -= 1;
        }

        result = result + (main_tank * 10);
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::distance_traveled(5, 10), 60);
        assert_eq!(Solution::distance_traveled(1, 2), 10);
    }
}
