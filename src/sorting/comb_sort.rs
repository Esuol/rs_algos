// # 梳排序
// # 梳排序是冒泡排序的改进版本，它的思想是通过增大冒泡排序中两个相邻元素之间的距离来消除逆序对。

pub fn comb_sort<T: Ord>(arr: &mut [T]) {
    let mut gap = arr.len();
    let shrink = 1.3;
    let mut sorted = false;

    while !sorted {
        gap = (gap as f32 / shrink).floor() as usize;
        if gap <= 1 {
            gap = 1;
            sorted = true;
        }
        for i in 0..arr.len() - gap {
            let j = i + gap;
            if arr[i] > arr[j] {
                arr.swap(i, j);
                sorted = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_sorted;

    #[test]
    fn descending() {
        let mut descending = vec![5, 4, 3, 2, 1];
        comb_sort(&mut descending);
        assert!(is_sorted(&descending));
    }

    #[test]
    fn ascending() {
        let mut ascending = vec![1, 2, 3, 4, 5];
        comb_sort(&mut ascending);
        assert!(is_sorted(&ascending));
    }

    #[test]
    fn basic() {
        let mut basic = vec![4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
        comb_sort(&mut basic);
        assert!(is_sorted(&basic));
    }
}
