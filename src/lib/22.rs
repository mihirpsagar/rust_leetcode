use std::collections::HashSet;

// Time taken: 13:27, 13:46 -> Acc, 17:55 -> Optimized
struct Solution {}

impl Solution {
    // Using recursion (DFS) -> 0ms
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::new();
        let mut string = String::new();
        let n = n as u8;

        Self::generate_parenthesis_rec(0, 0, n, &mut string, &mut result);

        return result;
    }

    pub fn generate_parenthesis_rec(
        open_count: u8,
        close_count: u8,
        n: u8,
        mut str: &mut String,
        mut result: &mut Vec<String>,
    ) {
        if open_count == n && close_count == n {
            result.push(str.clone());
            return;
        }

        if open_count < n {
            str.push('(');
            Self::generate_parenthesis_rec(open_count + 1, close_count, n, &mut str, &mut result);
            str.pop();
        }

        if close_count < open_count {
            str.push(')');
            Self::generate_parenthesis_rec(open_count, close_count + 1, n, &mut str, &mut result);
            str.pop();
        }
    }

    // First try -> 1 ms
    // pub fn generate_parenthesis(n: i32) -> Vec<String> {
    //     let mut result_set = HashSet::new();
    //     let mut arr = vec![0; (n * 2) as usize];

    //     loop {
    //         let count = Self::next_permutation(&mut arr);
    //         if count.1 == (n * 2) as u8 {
    //             break;
    //         }

    //         if count.0 != count.1 {
    //             continue;
    //         }

    //         if Self::is_valid_parenthesis(&arr) {
    //             result_set.insert(arr.clone());
    //         }
    //     }

    //     let mut result = Vec::new();
    //     for row in result_set {
    //         let mut str = String::new();
    //         for val in row {
    //             if val == 0 {
    //                 str.push('(');
    //             } else {
    //                 str.push(')');
    //             }
    //         }
    //         result.push(str);
    //     }

    //     return result;
    // }

    // pub fn is_valid_parenthesis(arr: &Vec<u8>) -> bool {
    //     let mut stack = Vec::new();
    //     for val in arr {
    //         if *val == 0 {
    //             stack.push(*val);
    //         } else {
    //             if stack.pop().is_none() {
    //                 return false;
    //             }
    //         }
    //     }

    //     return stack.is_empty();
    // }

    // pub fn next_permutation(arr: &mut Vec<u8>) -> (u8, u8) {
    //     let mut idx = 0;
    //     let mut zero_count = 0;
    //     let mut one_count = 0;
    //     let mut carry = 1;
    //     while idx < arr.len() {
    //         let sum = arr[idx] + carry;
    //         arr[idx] = sum % 2;
    //         carry = sum / 2;

    //         if arr[idx] == 0 {
    //             zero_count += 1;
    //         } else {
    //             one_count += 1;
    //         }
    //         idx += 1;
    //     }

    //     return (zero_count, one_count);
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::generate_parenthesis(3),
            ["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
        assert_eq!(Solution::generate_parenthesis(1), ["()"]);
    }
}
