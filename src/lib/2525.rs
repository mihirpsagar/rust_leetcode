// Time taken: 23:28, 23:33 -> Wrong, 23:36 -> Acc
struct Solution {}

impl Solution {
    pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
        let mut is_bulky = false;
        let mut is_heavy = false;

        if length >= 10_000 || width >= 10_000 || height >= 10_000 || mass >= 10_000 {
            is_bulky = true;
        }

        if length as u64 * width as u64 * height as u64 >= 1_000_000_000 {
            is_bulky = true;
        }

        if mass >= 100 {
            is_heavy = true;
        }

        if is_bulky && is_heavy {
            return String::from("Both");
        }

        if is_bulky {
            return String::from("Bulky");
        }

        if is_heavy {
            return String::from("Heavy");
        }

        return String::from("Neither");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::categorize_box(1000, 35, 700, 300), "Heavy");
        assert_eq!(Solution::categorize_box(200, 50, 800, 50), "Neither");
        assert_eq!(Solution::categorize_box(2909, 3968, 3272, 727), "Both");
    }
}
