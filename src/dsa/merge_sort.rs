use rand::Rng;

pub fn merge_sort(mut arr: &mut Vec<i32>) {
    let len = arr.len();
    merge_sort_rec(&mut arr, 0, len);
}

pub fn merge_sort_rec(mut arr: &mut Vec<i32>, left: usize, right: usize) {
    if left >= right || right - left == 1 {
        return;
    }

    let mid = left + (right - left) / 2;
    merge_sort_rec(&mut arr, left, mid);
    merge_sort_rec(&mut arr, mid, right);

    let mut arr2 = Vec::new();
    let mut idx1 = left;
    let mut idx2 = mid;

    while idx1 < mid && idx2 < right {
        if arr[idx1] < arr[idx2] {
            arr2.push(arr[idx1]);
            idx1 += 1;
        } else {
            arr2.push(arr[idx2]);
            idx2 += 1;
        }
    }

    while idx1 < mid {
        arr2.push(arr[idx1]);
        idx1 += 1;
    }
    while idx2 < right {
        arr2.push(arr[idx2]);
        idx2 += 1;
    }

    idx1 = left;
    idx2 = 0;
    while idx1 < right {
        arr[idx1] = arr2[idx2];
        idx1 += 1;
        idx2 += 1;
    }
}

pub fn create_random_array() -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let len = rng.gen_range(0..=100) as usize;
    let mut arr = Vec::new();

    for _ in 0..len {
        arr.push(rng.gen_range(i32::MIN..=i32::MAX));
    }

    return arr;
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_merge_sort() {
        for _ in 0..10 {
            let mut arr = create_random_array();
            let mut arr2 = arr.clone();
            merge_sort(&mut arr);
            arr2.sort();
            assert_eq!(arr, arr2);
        }
    }
}
