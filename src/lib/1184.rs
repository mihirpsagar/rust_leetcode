// Time taken: 22:42, 22:47 -> Acc
struct Solution {}

impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let mut sum = (0, 0);
        let len = distance.len();
        let start = start as usize;
        let destination = destination as usize;
        let mut idx = start;

        while idx != destination {
            sum.0 += distance[idx];
            idx = (idx + 1) % len;
        }

        idx = destination;
        while idx != start {
            sum.1 += distance[idx];
            if sum.1 > sum.0 {
                break;
            }
            idx = (idx + 1) % len;
        }

        return std::cmp::min(sum.0, sum.1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 1),
            1
        );
        assert_eq!(
            Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 2),
            3
        );
        assert_eq!(
            Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 3),
            4
        );
    }
}
