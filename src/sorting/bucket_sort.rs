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
        super::bubble_sort(bucket);
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
    use crate::is_sorted;
    use super::*;

    #[test]
    fn empty() {
        let arr: [usize; 0] = [];
        let res = bucket_sort(&arr);
        assert!(is_sorted(&res));
    }
}
