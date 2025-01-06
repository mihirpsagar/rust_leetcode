use rand::Rng;

pub fn quick_sort(mut arr: &mut Vec<i32>) {
    let len = arr.len();
    quick_sort_rec(&mut arr, 0, len);
}

pub fn quick_sort_rec(mut arr: &mut Vec<i32>, start: usize, end: usize) {
    if start >= end {
        return;
    }

    let mut left = start;
    let mut idx = start;
    let pivot = end - 1;

    while idx < end - 1 {
        if arr[idx] < arr[pivot] {
            let tmp = arr[left];
            arr[left] = arr[idx];
            arr[idx] = tmp;
            left += 1;
        }

        idx += 1;
    }

    let tmp = arr[left];
    arr[left] = arr[pivot];
    arr[pivot] = tmp;

    quick_sort_rec(&mut arr, start, left);
    quick_sort_rec(&mut arr, left + 1, end);
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
    pub fn test_quick_sort() {
        for _ in 0..10 {
            let mut arr = create_random_array();
            let mut arr2 = arr.clone();
            arr2.sort();
            quick_sort(&mut arr);
            assert_eq!(arr, arr2);
        }
    }
}
