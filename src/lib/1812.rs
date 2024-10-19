// Time taken: 00:22, 00:29 -> Acc
struct Solution {}

impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        let start_black = vec!['a', 'c', 'e', 'g'];
        let coordinates = coordinates.chars().collect::<Vec<char>>();
        let col = coordinates[0];
        let row = u8::from_str_radix(&String::from(coordinates[1]), 10).unwrap();

        let mut color = if start_black.contains(&col) { 1 } else { 0 }; //1 black, 0 white
        color = (color + (row - 1)) % 2;

        return color == 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::square_is_white("a1".to_string()), false);
        assert_eq!(Solution::square_is_white("h3".to_string()), true);
        assert_eq!(Solution::square_is_white("c7".to_string()), false);
    }
}
