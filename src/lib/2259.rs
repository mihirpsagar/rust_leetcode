// Time taken: 02:08, 02:18 -> Acc
struct Solution {}

impl Solution {
    pub fn remove_digit(number: String, digit: char) -> String {
        let mut number = number.chars().collect::<Vec<char>>();
        let mut removed = false;
        let mut target_idx = 0;
        let mut idx = 0;

        while idx < number.len() {
            if number[idx] == digit {
                if idx + 1 < number.len() && number[idx + 1] > digit {
                    number.remove(idx);
                    removed = true;
                    break;
                }
                target_idx = idx;
            }
            idx += 1;
        }

        if !removed {
            number.remove(target_idx);
        }

        return number.iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::remove_digit("123".to_string(), '3'), "12");
        assert_eq!(Solution::remove_digit("1231".to_string(), '1'), "231");
        assert_eq!(Solution::remove_digit("551".to_string(), '5'), "51");
    }
}
