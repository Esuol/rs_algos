// 堆排序
fn heapify<T: PartialOrd>(arr: &mut [T], root: usize, end: usize) {
    let mut laggest = root;

    let left = 2 * root + 1;

    if left < end && arr[left] > arr[laggest] {
        laggest = left;
    }

    let right = 2 * root + 2;

    if right < end && arr[right] > arr[laggest] {
        laggest = right;
    }

    if laggest != root {
        arr.swap(root, laggest);
        heapify(arr, laggest, end);
    }
}
