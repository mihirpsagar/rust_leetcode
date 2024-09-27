// Time taken: 22:44, 22:47 -> Wrong, 22:49 -> Acc
struct Solution {}

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut sum = (0, 0);

        for ch in moves.chars() {
            if ch == 'U' {
                sum.0 += 1;
            } else if ch == 'D' {
                sum.0 -= 1;
            } else if ch == 'R' {
                sum.1 += 1;
            } else {
                sum.1 -= 1;
            }
        }

        return sum.0 == 0 && sum.1 == 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
