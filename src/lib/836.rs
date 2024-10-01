// Time taken: 20:13, 20:30 -> Wrong
struct Solution {}

impl Solution {
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        let mut result = true;

        for idx in 0..rec1.len() {
            result = result && (rec1[idx] == rec2[idx]);
        }
        if result {
            return true;
        }

        if rec1[0] >= rec2[0] && rec1[0] >= rec2[2] {
            return false;
        } else if rec1[2] <= rec2[0] && rec1[2] <= rec2[2] {
            return false;
        } else if rec1[1] >= rec2[1] && rec1[1] >= rec2[3] {
            return false;
        } else if rec1[3] <= rec2[1] && rec1[3] <= rec2[3] {
            return false;
        } else {
            return true;
        }
    }

    // pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
    //     let p1 = (rec1[0], rec1[1]);
    //     let p2 = (rec1[0], rec1[3]);
    //     let p3 = (rec1[2], rec1[3]);
    //     let p4 = (rec1[2], rec1[1]);

    //     let p5 = (rec2[0], rec2[1]);
    //     let p6 = (rec2[0], rec2[3]);
    //     let p7 = (rec2[2], rec2[3]);
    //     let p8 = (rec2[2], rec2[1]);

    //     if p1 == p5 && p2 == p6 && p3 == p7 && p4 == p8 {
    //         return true;
    //     }

    //     if Self::contains_in(p1, &rec2)
    //         || Self::contains_in(p2, &rec2)
    //         || Self::contains_in(p3, &rec2)
    //         || Self::contains_in(p4, &rec2)
    //     {
    //         return true;
    //     }

    //     if Self::contains_in(p5, &rec1)
    //         || Self::contains_in(p6, &rec1)
    //         || Self::contains_in(p7, &rec1)
    //         || Self::contains_in(p8, &rec1)
    //     {
    //         return true;
    //     }

    //     return false;
    // }

    // fn contains_in(p1: (i32, i32), rect: &Vec<i32>) -> bool {
    //     return p1.0 > rect[0] && p1.0 < rect[2] && p1.1 > rect[1] && p1.1 < rect[3];
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::is_rectangle_overlap(vec![0, 0, 2, 2], vec![1, 1, 3, 3]),
            true
        );
        assert_eq!(
            Solution::is_rectangle_overlap(vec![0, 0, 1, 1], vec![1, 0, 2, 1]),
            false
        );
        assert_eq!(
            Solution::is_rectangle_overlap(vec![0, 0, 2, 2], vec![2, 2, 3, 3]),
            false
        );
        assert_eq!(
            Solution::is_rectangle_overlap(vec![7, 8, 13, 15], vec![10, 8, 12, 20]),
            true
        );
    }
}
