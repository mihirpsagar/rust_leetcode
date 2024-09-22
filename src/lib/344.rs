// Time taken: 20:58, 20:59 -> Acc
struct Solution {}

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut start = 0;
        let mut end = s.len() - 1;

        while start < end {
            let tmp = s[start];
            s[start] = s[end];
            s[end] = tmp;

            start += 1;
            end -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
