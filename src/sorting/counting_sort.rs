pub fn counting_sort(arr: &mut [usize]) {
    let max = match arr.iter().max() {
        Some(max) => *max,
        None => return,
    };

    let mut count = vec![0; max + 1];
    let mut output = vec![0; arr.len()];

    for &number in arr.iter() {
        count[number] += 1;
    }

    for i in 1..=max {
        count[i] += count[i - 1];
    }

    for &number in arr.iter().rev() {
        output[count[number] - 1] = number;
        count[number] -= 1;
    }

    arr.copy_from_slice(&output);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_sorted;

    #[test]
    fn basic() {
        let mut basic = vec![4, 65, 2, 31, 0, 99, 2, 83, 782, 1];
        counting_sort(&mut basic);
        assert!(is_sorted(&basic));
    }

    #[test]
    fn counting_sort_pre_sorted() {
        let mut pre_sorted = vec![1, 2, 3, 4, 5];
        counting_sort(&mut pre_sorted);
        assert!(is_sorted(&pre_sorted));
    }

    #[test]
    fn generic_counting_sort() {
        let mut basic = vec![4, 65, 2, 31, 0, 99, 2, 83, 782, 1];
        counting_sort(&mut basic);
        assert!(is_sorted(&basic));
    }

    #[test]
    fn presorted_u64_counting_sort() {
        let mut pre_sorted = vec![1, 2, 3, 4, 5];
        counting_sort(&mut pre_sorted);
        assert!(is_sorted(&pre_sorted));
    }
}
