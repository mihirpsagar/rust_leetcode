use std::collections::HashSet;

// Time taken: 21:18
struct Solution {}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut hash_set: HashSet<i32> = HashSet::new();
        let mut is_duplicate: bool = false;
        for &num in nums.iter() {
            if hash_set.contains(&num) {
                is_duplicate = true;
                break;
            }
            hash_set.insert(num);
        }
        return is_duplicate;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
