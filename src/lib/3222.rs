// Time taken: 12:18, 12:22 -> Acc
struct Solution {}

impl Solution {
    pub fn winning_player(x: i32, y: i32) -> String {
        let val = std::cmp::min(x, y / 4);
        if val % 2 == 0 {
            return String::from("Bob");
        } else {
            return String::from("Alice");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::winning_player(2, 7), "Alice");
        assert_eq!(Solution::winning_player(4, 11), "Bob");
    }
}
