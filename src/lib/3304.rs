// Time taken: 15:27, 15:33 -> Acc
struct Solution {}

impl Solution {
    pub fn kth_character(k: i32) -> char {
        let mut word = vec!['a'];
        let k = k as usize;
        while word.len() < k {
            Self::generate(&mut word);
        }

        return word[k - 1];
    }

    pub fn generate(arr: &mut Vec<char>) {
        let mut new_arr = Vec::new();
        let mut idx = 0;
        let a_ascii = 'a' as u8;

        while idx < arr.len() {
            new_arr.push((((arr[idx] as u8 - a_ascii + 1) % 26) + a_ascii) as char);
            idx += 1;
        }

        arr.append(&mut new_arr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::kth_character(5), 'b');
        assert_eq!(Solution::kth_character(10), 'c');
    }
}
