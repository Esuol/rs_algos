pub fn m_s<T>(arr: &mut [T])
where
    T: PartialOrd + Clone + Default,
{
    if arr.len() > 1 {
        merge_sort_range(arr, 0, arr.len() - 1);
    }
}

fn merge_sort_range<T>(arr: &mut [T], start: usize, end: usize)
where
    T: PartialOrd + Clone + Default,
{
    if start < end {
        let mid = (start + end) / 2;
        merge_sort_range(arr, start, mid);
        merge_sort_range(arr, mid + 1, end);
        merge(arr, start, mid, end);
    }
}

fn merge<T>(arr: &mut [T], start: usize, mid: usize, end: usize)
where
    T: PartialOrd + Clone + Default,
{
    let left = arr[start..=mid].to_vec();
    let right = arr[mid + 1..=end].to_vec();
    let mut i = 0;
    let mut j = 0;
    let mut k = start;
    while i < left.len() && j < right.len() {
        if (left[i] <= right[j]) {
            arr[k] = left[i].clone();
            i += 1;
        } else {
            arr[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_sorted;

    #[test]
    fn basic() {
        let mut basic = vec![4, 65, 2, 31, 0, 99, 2, 83, 782, 1];
        m_s(&mut basic);
        assert!(is_sorted(&basic));
    }

    #[test]
    fn tset_nums_vec() {
        let mut nums = vec![4, 65, 2, 31, 0, 99, 2, 83, 782, 1];
        m_s(&mut nums);
        assert_eq!(nums, vec![0, 1, 2, 2, 4, 31, 65, 83, 99, 782]);
    }

    #[test]
    fn test_str_vec() {
        let mut str_vec = vec!["c", "a", "e", "b", "d"];
        m_s(&mut str_vec);
        assert_eq!(str_vec, vec!["a", "b", "c", "d", "e"]);
    }
}
