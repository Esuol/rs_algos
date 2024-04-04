pub fn selection_sort<T: PartialOrd>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let size = arr.len();
    for i in 0..(size - 1) {
        let mut min_index = i;
        for j in (i + 1)..size {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }

        if min_index != i {
            arr.swap(i, min_index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_vec() {
        let mut empty_vec: Vec<String> = vec![];
        selection_sort(&mut empty_vec);
        assert_eq!(empty_vec, Vec::<String>::new());
    }

    #[test]
    fn test_number_vec() {
        let mut number_vec = vec![4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
        selection_sort(&mut number_vec);
        assert_eq!(number_vec, vec![-31, 0, 1, 2, 2, 4, 65, 83, 99, 782]);
    }

    #[test]
    fn test_string_vec() {
        let mut string_vec = vec!["beach", "hotel", "airplane", "car", "house", "art"];
        selection_sort(&mut string_vec);
        assert_eq!(
            string_vec,
            vec!["airplane", "art", "beach", "car", "hotel", "house"]
        );
    }
}
