// Time taken: 01:27, 01:29 -> Acc
struct Solution {}

impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        let mut result = Vec::new();
        for _ in 0..n {
            result.push('a');
        }
        if n % 2 == 0 {
            result.pop();
            result.push('b');
        }

        return result.iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
