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
