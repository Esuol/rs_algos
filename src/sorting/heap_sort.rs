// 堆排
pub fn heap_sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();

    // 构建大根堆
    for i in (0..len / 2).rev() {
        heapify(arr, i, len);
    }

    // 每轮循环将堆顶元素（也就是最大元素）放到最后
    for i in (1..len).rev() {
        arr.swap(0, i);
        // 恢复大根堆
        heapify(arr, 0, i);
    }
}

fn heapify<T: PartialOrd>(arr: &mut [T], root: usize, end: usize) {
    let mut largest = root;

    let left = 2 * root + 1;

    if left < end && arr[left] > arr[largest] {
        largest = left;
    }

    let right = 2 * root + 2;

    if right < end && arr[right] > arr[largest] {
        largest = right;
    }

    if largest != root {
        arr.swap(root, largest);
        heapify(arr, largest, end);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_sorted;

    #[test]
    fn basic() {
        let mut basic = vec![4, 65, 2, 31, 0, 99, 2, 83, 782, 1];
        heap_sort(&mut basic);
        assert!(is_sorted(&basic));
    }

    #[test]
    fn test_empty_vec() {
        let mut empty_vec: Vec<String> = vec![];
        heap_sort(&mut empty_vec);
        assert_eq!(empty_vec, Vec::<String>::new());
    }

    #[test]
    fn test_string_vec() {
        let mut string_vec = vec![
            "rust",
            "is",
            "fast",
            "and",
            "memory-efficient",
            "with",
            "no",
            "runtime",
            "or",
            "garbage",
            "collector",
        ];
        heap_sort(&mut string_vec);
        assert!(is_sorted(&string_vec));
    }

    #[test]
    fn test_number_vec() {
        let mut number_vec = vec![4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
        heap_sort(&mut number_vec);
        assert_eq!(number_vec, vec![-31, 0, 1, 2, 2, 4, 65, 83, 99, 782]);
    }
}
