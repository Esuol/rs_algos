// 地精排序
// 地精排序（Gnome Sort）是一种简单的排序算法，其工作原理类似于插入排序，但是通过交换相邻的元素来移动元素到正确的位置。

use std::cmp::{PartialEq, PartialOrd};

pub fn gnome_sort<T>(arr: &[T]) -> Vec<T>
where
    T: PartialEq + PartialOrd + Clone,
{
    let mut arr = arr.to_vec();
    let mut i = 1;
    let mut j = 2;

    while i < arr.len() {
        if arr[i - 1] <= arr[i] {
            i = j;
            j += 1;
        } else {
            arr.swap(i - 1, i);
            i -= 1;
            if i == 0 {
                i = j;
                j += 1;
            }
        }
    }
    arr
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_sorted;

    #[test]
    fn basic() {
        let basic = vec![4, 65, 2, 31, 0, 99, 2, 83, 782, 1];
        let sorted = gnome_sort(&basic);
        assert!(is_sorted(&sorted));
    }

    #[test]
    fn already_sorted() {
        let sorted = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let gnome_sorted = gnome_sort(&sorted);
        assert_eq!(sorted, gnome_sorted);
    }

    #[test]
    fn odd_number_of_elements() {
        let odd = vec![4, 65, 2, 31, 0, 99, 2, 83, 782];
        let sorted = gnome_sort(&odd);
        assert!(is_sorted(&sorted));
    }

    #[test]
    fn empty() {
        let empty: Vec<i32> = vec![];
        let sorted = gnome_sort(&empty);
        assert!(sorted.is_empty());
    }

    #[test]
    fn one_element() {
        let one = vec![0];
        let sorted = gnome_sort(&one);
        assert_eq!(one, sorted);
    }
}
