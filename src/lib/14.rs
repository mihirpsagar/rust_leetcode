// Time taken: 20:51, 21:08

struct Solution {}

impl Solution {
    // My solution
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut result: String = strs.get(0).unwrap().to_string();

        for str in strs.iter().skip(1) {
            let mut new_result = String::from("");
            let mut result_iter = result.chars();
            for ch in str.chars() {
                if let Some(ch2) = result_iter.next() {
                    if ch != ch2 {
                        break;
                    }
                    new_result += &String::from(ch);
                }
            }
            result = new_result;
            if result.len() == 0 {
                break;
            }
        }

        return result;
    }

    // Other solution
    // pub fn longest_common_prefix(strs: Vec<String>) -> String {
    //     if strs.len() == 0 {
    //         return String::new();
    //     }

    //     let mut result: String = strs.get(0).unwrap().to_string();

    //     for str in strs.iter() {
    //         while !str.starts_with(&result) {
    //             result.pop();
    //             if result.is_empty() {
    //                 return result;
    //             }
    //         }
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
            Solution::longest_common_prefix(vec!["flower", "flow", "flight"]),
            "fl"
        );
        assert_eq!(
            Solution::longest_common_prefix(vec!["dog", "racecar", "car"]),
            ""
        );
    }
}
