// Time taken: 16:30, 16:34 -> Acc
struct Solution {}

impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        if n % 4 == 0 {
            return false;
        } else {
            return true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
