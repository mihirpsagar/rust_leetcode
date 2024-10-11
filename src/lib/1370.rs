// Time taken: 01:04, 01:15, 01:24 -> Acc
struct Solution {}

impl Solution {
    pub fn sort_string(s: String) -> String {
        let mut result = Vec::new();
        let mut arr = s.chars().collect::<Vec<char>>();
        arr.sort();

        while !arr.is_empty() {
            if arr.len() == 0 {
                break;
            }
            let mut last_ch = arr[0];
            result.push(last_ch);
            arr.remove(0);
            loop {
                let mut idx = 0;
                let mut swapped = false;
                while idx < arr.len() {
                    if arr[idx] > last_ch {
                        result.push(arr[idx]);
                        last_ch = arr[idx];
                        arr.remove(idx);
                        swapped = true;
                        break;
                    }
                    idx += 1;
                }
                if !swapped {
                    break;
                }
            }

            if arr.len() == 0 {
                break;
            }
            last_ch = arr[arr.len() - 1];
            result.push(last_ch);
            arr.remove(arr.len() - 1);
            loop {
                if arr.len() == 0 {
                    break;
                }
                let mut idx = arr.len() - 1;
                let mut swapped = false;
                loop {
                    if arr[idx] < last_ch {
                        result.push(arr[idx]);
                        last_ch = arr[idx];
                        arr.remove(idx);
                        swapped = true;
                        break;
                    }
                    if idx == 0 {
                        break;
                    }
                    idx -= 1;
                }
                if !swapped {
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
            Solution::sort_string("aaaabbbbcccc".to_string()),
            "abccbaabccba"
        );
        assert_eq!(Solution::sort_string("rat".to_string()), "art");
    }
}
