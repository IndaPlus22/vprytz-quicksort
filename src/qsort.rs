// quicksort implementation

pub fn quicksort(arr: &mut [i32]) {
    qsort(arr, 0, (arr.len() - 1) as isize);
}

fn insertion_sort(arr: &mut [i32], low: isize, high: isize) {
    for i in low + 1..=high {
        let mut j = i;
        while j > low && arr[j as usize] < arr[(j - 1) as usize] {
            arr.swap(j as usize, (j - 1) as usize);
            j -= 1;
        }
    }
}

fn qsort(arr: &mut [i32], low: isize, high: isize) {
    if low < high {
        if high - low < 50 {
            insertion_sort(arr, low, high);
        } else {
            let p = partition(arr, low, high);
            qsort(arr, low, p - 1);
            qsort(arr, p + 1, high);
        }
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
    fn test_qsort_with_larger() {
        let mut arr = [
            -1, -10, -30, -50, 5, 4, 3, 2, 1, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
            20,
        ];
        quicksort(&mut arr);
        assert_eq!(
            arr,
            [
                -50, -30, -10, -1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
                19, 20
            ]
        );
    }

    #[test]
    fn test_insertion_sort() {
        let mut arr = [5, 4, 3, 2, 1];
        insertion_sort(&mut arr, 0, 4);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }
}
