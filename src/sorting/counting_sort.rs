// 记数排序
pub fn counting_sort<T: Ord>(arr: &mut [T]) {
    let max = match arr.iter().max() {
        Some(max) => max,
        None => return,
    };

    let mut count = vec![0; max + 1];
    let mut output = vec![0, arr.len()];

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
