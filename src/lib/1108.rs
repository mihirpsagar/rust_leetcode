// Time taken: 23:55, 23:56 -> Acc
struct Solution {}

impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        return address.replace(".", "[.]");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::defang_i_paddr("1.1.1.1".to_string()),
            "1[.]1[.]1[.]1"
        );
        assert_eq!(
            Solution::defang_i_paddr("255.100.50.0".to_string()),
            "255[.]100[.]50[.]0"
        );
    }
}
