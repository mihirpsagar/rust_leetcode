// Time taken: 12:56, 12:59 -> Acc
struct Solution {}

impl Solution {
    pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
        let mut val = 0;
        for word in commands {
            if word == "LEFT" {
                val -= 1;
            } else if word == "RIGHT" {
                val += 1;
            } else if word == "UP" {
                val -= n;
            } else {
                val += n;
            }
        }

        return val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::final_position_of_snake(2, vec!["RIGHT".to_string(), "DOWN".to_string()]),
            3
        );
        assert_eq!(
            Solution::final_position_of_snake(
                3,
                vec!["DOWN".to_string(), "RIGHT".to_string(), "UP".to_string()]
            ),
            1
        );
    }
}
