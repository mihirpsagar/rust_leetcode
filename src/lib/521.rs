use std::cmp::max;

// Time taken: 00:07, 00:10 -> Acc
struct Solution {}

impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        if a == b {
            return -1;
        } else {
            return max(a.len(), b.len()) as i32;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
