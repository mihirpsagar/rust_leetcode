// Time taken: 21:33, 21:40, 21:49 -> Acc
struct Solution {}

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut min_cost = vec![None; cost.len()];
        min_cost[0] = Some(cost[0]);
        min_cost[1] = Some(cost[1]);

        return Self::min_climb_rec(&cost, &mut min_cost, cost.len());
    }

    fn min_climb_rec(cost: &Vec<i32>, mut min_cost: &mut Vec<Option<i32>>, idx: usize) -> i32 {
        if let Some(min_cost) = min_cost.get(idx).and_then(|x| *x) {
            return min_cost;
        }

        if idx < 2 {
            return cost[idx];
        }

        let &curr_cost = cost.get(idx).unwrap_or(&0);
        let one_step_cost = curr_cost + Self::min_climb_rec(&cost, &mut min_cost, idx - 1);
        let two_step_cost = curr_cost + Self::min_climb_rec(&cost, &mut min_cost, idx - 2);
        let result = std::cmp::min(one_step_cost, two_step_cost);

        if idx < min_cost.len() {
            min_cost[idx] = Some(result);
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
        assert_eq!(
            Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }
}
