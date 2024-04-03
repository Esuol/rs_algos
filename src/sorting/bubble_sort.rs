// 冒泡排序
pub fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let size = arr.len();

    if size <= 1 {
        return;
    }

    for i in 0..(size - 1) {
        let mut swapped = false;

        for j in 1..(size - i) {
            if arr[j] < arr[j - 1] {
                arr.swap(j, j - 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}
