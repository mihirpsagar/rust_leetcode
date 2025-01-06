pub fn binary_search(arr: Vec<i32>, target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    return None;
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_binary_search() {
        assert_eq!(binary_search(vec![100], 100), Some(0));
        assert_eq!(binary_search(vec![100], 50), None);
        assert_eq!(binary_search(vec![100], 10), None);
        assert_eq!(binary_search(vec![10, 20, 30, 40, 50], 10), Some(0));
        assert_eq!(binary_search(vec![10, 20, 30, 40, 50], 20), Some(1));
        assert_eq!(binary_search(vec![10, 20, 30, 40, 50], 30), Some(2));
        assert_eq!(binary_search(vec![10, 20, 30, 40, 50], 40), Some(3));
        assert_eq!(binary_search(vec![10, 20, 30, 40, 50], 50), Some(4));
        assert_eq!(binary_search(vec![10, 20, 30, 40, 50], 0), None);
        assert_eq!(binary_search(vec![10, 20, 30, 40, 50], 70), None);
    }
}
