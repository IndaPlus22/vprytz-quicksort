// quicksort implementation

pub fn quicksort(arr: &mut [i32]) {
    qsort(arr, 0, (arr.len() - 1) as isize);
}

fn qsort(arr: &mut [i32], low: isize, high: isize) {
    if low < high {
        let p = partition(arr, 0, high);
        qsort(arr, low, p - 1);
        qsort(arr, p + 1, high);
    }
}

// Lomuto algorithm:
fn partition(arr: &mut [i32], low: isize, high: isize) -> isize {
    let pivot = high;
    let mut i = low as isize - 1;

    for j in low..=high - 1 {
        if arr[j as usize] <= arr[pivot as usize] {
            i += 1;
            arr.swap(i as usize, j as usize);
        }
    }
    arr.swap((i + 1) as usize, pivot as usize);
    i + 1
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qsort() {
        let mut arr = [5, 4, 3, 2, 1];
        quicksort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_with_larger() {
        let mut arr = [
            5, 4, 3, 2, 1, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ];
        quicksort(&mut arr);
        assert_eq!(
            arr,
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
        );
    }
}
