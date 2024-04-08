// 插入排序
pub fn insertion_sort<T: PartialOrd>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

pub fn insertion_sort_binary_search<T: Ord>(arr: &mut [T]) {
    // 从第二个元素开始排序
    for i in 1..arr.len() {
        // 利用二分查找获取 arr[i] 应该插入的位置
        let pos = arr[..i].binary_search(&arr[i]).unwrap_or_else(|x| x);
        let mut j = i;
        while j > pos {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_sorted;

    #[test]
    fn basic() {
        let mut basic = vec![4, 65, 2, 31, 0, 99, 2, 83, 782, 1];
        insertion_sort(&mut basic);
        assert!(is_sorted(&basic));
    }

    #[test]
    fn basic_binary_search() {
        let mut basic = vec![4, 65, 2, 31, 0, 99, 2, 83, 782, 1];
        insertion_sort_binary_search(&mut basic);
        assert!(is_sorted(&basic));
    }

    #[test]
    fn empty() {
        let mut empty: Vec<i32> = vec![];
        insertion_sort(&mut empty);
        assert!(empty.is_empty());
    }

    #[test]
    fn empty_binary_search() {
        let mut empty: Vec<i32> = vec![];
        insertion_sort_binary_search(&mut empty);
        assert!(empty.is_empty());
    }

    #[test]
    fn already_sorted() {
        let mut sorted = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        insertion_sort(&mut sorted);
        assert!(is_sorted(&sorted));
    }

    #[test]
    fn already_sorted_binary_search() {
        let mut sorted = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        insertion_sort_binary_search(&mut sorted);
        assert!(is_sorted(&sorted));
    }
}
