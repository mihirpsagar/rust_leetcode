// Time taken: 20:35, 20:49

struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result: i32 = 0;

        let mut s_iter = s.chars().peekable();
        while let Some(ch) = s_iter.next() {
            match (ch, s_iter.peek()) {
                ('I', Some('V')) => result -= 1,
                ('I', Some('X')) => result -= 1,
                ('X', Some('L')) => result -= 10,
                ('X', Some('C')) => result -= 10,
                ('C', Some('D')) => result -= 100,
                ('C', Some('M')) => result -= 100,
                ('I', _) => result += 1,
                ('V', _) => result += 5,
                ('X', _) => result += 10,
                ('L', _) => result += 50,
                ('C', _) => result += 100,
                ('D', _) => result += 500,
                ('M', _) => result += 1000,
                _ => panic!("Invalid character found!"),
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
        assert_eq!(Solution::roman_to_int(String::from("III")), 3);
        assert_eq!(Solution::roman_to_int(String::from("LVIII")), 58);
        assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")), 1994);
    }
}
