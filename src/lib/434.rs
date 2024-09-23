// Time taken: 14:11, 14:16 -> Acc
struct Solution {}

impl Solution {
    pub fn count_segments(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }

        let s_vec: Vec<&str> = s.trim().split_whitespace().collect();

        return s_vec.len() as i32;
    }

    // pub fn count_segments(s: String) -> i32 {
    //     let mut result = 0;
    //     let s_vec: Vec<char> = s.chars().collect();
    //     let mut word = false;

    //     if s_vec.len() == 0 {
    //         return result;
    //     }

    //     for idx in 0..s_vec.len() {
    //         if s_vec[idx] != ' ' {
    //             word = true;
    //         } else {
    //             if word {
    //                 word = false;
    //                 result += 1;
    //             }
    //         }
    //     }

    //     if s_vec[s_vec.len() - 1] != ' ' {
    //         result += 1;
    //     }

    //     return result;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::count_segments(String::from("Hello, my name is John")),
            5
        );
        assert_eq!(Solution::count_segments(String::from("Hello")), 1);
    }
}
