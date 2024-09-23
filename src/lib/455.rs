// Time taken: 15:18, 15:26 -> Acc but O(n^2)
struct Solution {}

impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut g = g;
        let mut s = s;
        g.sort_by(|a, b| b.cmp(a));
        s.sort_by(|a, b| b.cmp(a));
        let mut result = 0;
        let mut g_idx = 0;
        let mut s_idx = 0;

        while g_idx < g.len() && s_idx < s.len() {
            if g[g_idx] <= s[s_idx] {
                result += 1;
                s_idx += 1;
            }
            g_idx += 1;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_content_children(vec![1, 2, 3], vec![1, 1]),
            1
        );
        assert_eq!(
            Solution::find_content_children(vec![1, 2], vec![1, 2, 3]),
            2
        );
    }
}
