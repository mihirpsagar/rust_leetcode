// Time taken: 22:59, 23:10, 23:19 -> Acc
struct Solution {}

impl Solution {
    pub fn maximum_units(mut box_types: Vec<Vec<i32>>, mut truck_size: i32) -> i32 {
        box_types.sort_by(|a, b| b[1].cmp(&a[1]));

        let mut result = 0;
        let mut idx = 0;
        while truck_size > 0 && idx < box_types.len() {
            let val = &box_types[idx];
            result = result + (val[1] * std::cmp::min(val[0], truck_size));
            truck_size -= std::cmp::min(val[0], truck_size);
            idx += 1;
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
            Solution::maximum_units(vec![vec![1, 3], vec![2, 2], vec![3, 1]], 4),
            8
        );
        assert_eq!(
            Solution::maximum_units(vec![vec![5, 10], vec![2, 5], vec![4, 7], vec![3, 9]], 10),
            91
        );
    }
}
