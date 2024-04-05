// # 梳排序
// # 梳排序是冒泡排序的改进版本，它的思想是通过增大冒泡排序中两个相邻元素之间的距离来消除逆序对。

pub fn comp_sort<T: Ord>(arr: &mut [T]) {
    let mut gap = arr.len();
    let shrink = 1.3;
    let mut sorted = false;

    while !sorted {
        gap = (gap as f64 / shrink).floor() as usize;

        if gap <= 1 {
            gap = 1;
            sorted = true;
        }

        for i in 0..(arr.len() - gap) {
            if arr[i] > arr[i + gap] {
                arr.swap(i, i + gap);
                sorted = false;
            }
        }
    }
}
