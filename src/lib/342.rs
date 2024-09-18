// Time taken: 17:53, 17:57 -> Wrong, 18:02 -> Acc
struct Solution {}

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        return n >= 1 && (n as f64).log(4f64) % 1f64 == 0f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
