// Time taken: 13:41, 13:43 -> Acc
struct Solution {}

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut result = Vec::new();
        for num in 1..=n {
            let mut str = String::new();
            if num % 3 == 0 {
                str += &"Fizz";
            }
            if num % 5 == 0 {
                str += &"Buzz";
            }
            if str.is_empty() {
                result.push(num.to_string());
            } else {
                result.push(str);
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
        assert_eq!(Solution::fizz_buzz(3), vec!["1", "2", "Fizz"]);
        assert_eq!(Solution::fizz_buzz(5), vec!["1", "2", "Fizz", "4", "Buzz"]);
        assert_eq!(
            Solution::fizz_buzz(15),
            vec![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ]
        )
    }
}
