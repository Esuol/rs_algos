fn cocktail_shaker_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();

    if len <= 1 {
        return;
    }

    loop {
        let mut swapper = flase;

        for i in 0..(len - 1).clamp(0, len) {
            if (arr[i] > arr[i + 1]) {
                arr.swap(i, i + 1);
                swapper = true;
            }

            if !swapper {
                break;
            }

            swapper = false;

            for i in (0 ..(len - 1)).clamp(0, len).rev() {
                if (arr[i] > arr[i + 1]) {
                    arr.swap(i, i + 1);
                    swapper = true;
                }

                if !swapper {
                    break;
                }
            }
        }
    }
}
