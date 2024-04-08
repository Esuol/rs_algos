// 桶排序
pub fn bucket_sort(arr: &[usize]) -> Vec<usize> {
    if arr.is_empty() {
        return vec![];
    }

    let max = *arr.iter().max().unwrap();
    let len = arr.len();
    let mut buckets = vec![vec![]; len + 1];

    for x in arr {
        buckets[*x * len / max].push(*x);
    }

    for bucket in buckets.iter_mut() {
        super::super::bubble_sort(bucket);
    }

    let mut result = vec![];

    for bucket in buckets {
        for x in bucket {
            result.push(x);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_sorted;

    #[test]
    fn empty() {
        let arr: [usize; 0] = [];
        let res = bucket_sort(&arr);
        assert!(is_sorted(&res));
    }

    #[test]
    fn one_element() {
        let arr = [1];
        let res = bucket_sort(&arr);
        assert!(is_sorted(&res));
    }

    #[test]
    fn already_sorted() {
        let arr = [11, 8, 21];
        let res = bucket_sort(&arr);
        assert!(is_sorted(&res));
    }

    #[test]
    fn basic() {
        let arr = [11, 8, 21, 5, 10, 3, 13, 2, 34];
        let res = bucket_sort(&arr);
        assert!(is_sorted(&res));
    }
}
