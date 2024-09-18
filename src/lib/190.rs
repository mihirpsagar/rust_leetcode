// Time taken: 12:54, 12:58 -> Acc
struct Solution {}

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        return u32::from_str_radix(&format!("{x:032b}").chars().rev().collect::<String>(), 2)
            .unwrap();
    }

    // pub fn reverse_bits(x: u32) -> u32 {
    //     return x.reverse_bits();
    // }

    // pub fn reverse_bits(x: u32) -> u32 {
    //     return x.rotate_left(31);
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::reverse_bits(43261596), 964176192);
        assert_eq!(Solution::reverse_bits(4294967293), 3221225471);
    }
}
