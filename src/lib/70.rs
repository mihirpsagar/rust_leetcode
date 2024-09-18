use std::collections::HashMap;

// Time taken: 22:39, 20:47
struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        return Solution::climb_stairs_rec(n, &mut map);
    }

    pub fn climb_stairs_rec(n: i32, mut map: &mut HashMap<i32, i32>) -> i32 {
        if n < 3 {
            return n;
        }

        if map.contains_key(&n) {
            return *map.get(&n).unwrap();
        }

        let result = Solution::climb_stairs_rec(n - 1, &mut map)
            + Solution::climb_stairs_rec(n - 2, &mut map);
        map.insert(n, result);

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
    }
}
