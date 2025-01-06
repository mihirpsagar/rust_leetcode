use rand::Rng;

pub fn heap_sort(mut arr: &mut Vec<i32>) {
    if arr.len() < 2 {
        return;
    }
    let len = arr.len();

    let mut idx = len / 2;
    loop {
        heapify(&mut arr, idx, len);
        if idx == 0 {
            break;
        }
        idx -= 1;
    }

    idx = len - 1;
    loop {
        let tmp = arr[0];
        arr[0] = arr[idx];
        arr[idx] = tmp;
        heapify(&mut arr, 0, idx);
        if idx <= 1 {
            break;
        }
        idx -= 1;
    }
}

pub fn heapify(mut arr: &mut Vec<i32>, n: usize, len: usize) {
    let l = (2 * n) + 1;
    let r = (2 * n) + 2;
    let mut largest = n;

    if l < len && arr[l] > arr[largest] {
        largest = l;
    }

    if r < len && arr[r] > arr[largest] {
        largest = r;
    }

    if largest != n {
        let tmp = arr[n];
        arr[n] = arr[largest];
        arr[largest] = tmp;

        heapify(&mut arr, largest, len);
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
    pub fn test_heap_sort() {
        for _ in 0..10 {
            let mut arr = create_random_array();
            let mut arr2 = arr.clone();
            heap_sort(&mut arr);
            arr2.sort();
            assert_eq!(arr, arr2);
        }
    }
}
