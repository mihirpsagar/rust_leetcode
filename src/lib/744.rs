// Time taken: 21:25, 21:29

use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let mut left = 0;
        let mut right = letters.len();

        while left < right {
            let mid = left + (right - left) / 2;
            match letters[mid].cmp(&target) {
                Ordering::Equal | Ordering::Less => {
                    left = mid + 1;
                }
                Ordering::Greater => {
                    right = mid;
                }
            }
        }

        if left == letters.len() {
            return letters[0];
        }

        return letters[left];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'a'),
            'c'
        );
        assert_eq!(
            Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'c'),
            'f'
        );
        assert_eq!(
            Solution::next_greatest_letter(vec!['x', 'x', 'y', 'y'], 'z'),
            'x'
        );
    }
}
