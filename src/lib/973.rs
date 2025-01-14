// Time taken: 13:14, 13:29 -> Acc
// mod dsa;

use std::{
    cell::RefCell,
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    rc::Rc,
};

struct Solution {}

struct Point {
    distance: f64,
    coords: (i32, i32),
}

impl Point {
    fn new(distance: f64, coords: (i32, i32)) -> Self {
        return Self {
            distance: distance,
            coords: coords,
        };
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.distance < other.distance {
            return Ordering::Less;
        } else if self.distance == other.distance {
            return Ordering::Equal;
        } else {
            return Ordering::Greater;
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Eq for Point {}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return self.distance == other.distance;
    }
}

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut max_heap: BinaryHeap<Point> = BinaryHeap::new();
        let k = k as usize;
        let mut result = Vec::new();

        for point in points {
            let v1 = point[0].pow(2);
            let v2 = point[1].pow(2);
            let distance = ((v1 + v2) as f64).sqrt();

            if max_heap.len() < k {
                max_heap.push(Point::new(distance, (point[0], point[1])));
            } else {
                if max_heap.peek().unwrap().distance > distance {
                    max_heap.pop();
                    max_heap.push(Point::new(distance, (point[0], point[1])));
                }
            }
        }

        while let Some(point) = max_heap.pop() {
            result.push(vec![point.coords.0, point.coords.1]);
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
            Solution::k_closest(vec![vec![1, 3], vec![-2, 2]], 1),
            [[-2, 2]]
        );
    }
}
