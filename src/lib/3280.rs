// Time taken: 15:09, 15:17 -> Acc
struct Solution {}

impl Solution {
    pub fn convert_date_to_binary(date: String) -> String {
        let date = date.chars().collect::<Vec<char>>();
        let mut result = Vec::new();

        let mut year = Self::get_binary(&date[0..4]);
        let mut month = Self::get_binary(&date[5..7]);
        let mut date = Self::get_binary(&date[8..]);

        result.append(&mut year);
        result.push('-');
        result.append(&mut month);
        result.push('-');
        result.append(&mut date);

        return result.iter().collect();
    }

    pub fn get_binary(arr: &[char]) -> Vec<char> {
        let mut val = 0;
        let mut result = Vec::new();
        let mut idx = 0;
        while idx < arr.len() {
            val = (val * 10) + (arr[idx] as u8 - '0' as u8) as u32;
            idx += 1;
        }

        while val > 0 {
            if val % 2 == 0 {
                result.push('0');
            } else {
                result.push('1');
            }
            val /= 2;
        }

        result.reverse();
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::convert_date_to_binary("2080-02-29".to_string()),
            "100000100000-10-11101"
        );
        assert_eq!(
            Solution::convert_date_to_binary("1900-01-01".to_string()),
            "11101101100-1-1"
        );
    }
}
