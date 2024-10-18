// Time taken: 22:36, 22:44 -> Acc
struct Solution {}

impl Solution {
    pub fn reformat_number(number: String) -> String {
        let mut result = Vec::new();
        let mut arr = Vec::new();

        for ch in number.chars() {
            if ch == ' ' || ch == '-' {
                continue;
            }
            arr.push(ch);
        }

        let mut idx = 0;
        while idx < arr.len() {
            if idx + 4 < arr.len() {
                result.push(arr[idx]);
                result.push(arr[idx + 1]);
                result.push(arr[idx + 2]);
                result.push('-');
                idx += 3;
            } else {
                if idx + 3 < arr.len() {
                    result.push(arr[idx]);
                    result.push(arr[idx + 1]);
                    result.push('-');
                    result.push(arr[idx + 2]);
                    result.push(arr[idx + 3]);
                    break;
                } else {
                    let mut idx2 = idx;
                    while idx2 < arr.len() {
                        result.push(arr[idx2]);
                        idx2 += 1;
                    }
                    break;
                }
            }
        }

        return result.iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::reformat_number("1-23-45 6".to_string()),
            "123-456"
        );
        assert_eq!(
            Solution::reformat_number("123 4-567".to_string()),
            "123-45-67"
        );
        assert_eq!(
            Solution::reformat_number("123 4-5678".to_string()),
            "123-456-78"
        );
    }
}
