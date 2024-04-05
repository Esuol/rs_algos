pub fn cocktail_shaker_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();

    if len <= 1 {
        return;
    }

    loop {
        let mut swapped = false;

        for i in 0..(len - 1).clamp(0, len) {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }

            if !swapped {
                break;
            }

            swapped = false;

            for i in (0..(len - 1).clamp(0, len)).rev() {
                if arr[i] > arr[i + 1] {
                    arr.swap(i, i + 1);
                    swapped = true;
                }

                if !swapped {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_sorted;

    #[test]
    fn test_empty_vec() {
        let mut empty_vec: Vec<String> = vec![];
        cocktail_shaker_sort(&mut empty_vec);
        assert_eq!(empty_vec, Vec::<String>::new());
    }

    #[test]
    fn one_element() {
        let mut one_element = vec![1];
        cocktail_shaker_sort(&mut one_element);
        assert!(is_sorted(&one_element));
    }

    #[test]
    fn pre_sorted() {
        let mut pre_sorted = vec![1, 2, 3, 4, 5];
        cocktail_shaker_sort(&mut pre_sorted);
        assert!(is_sorted(&pre_sorted));
    }

    #[test]
    fn test_number_vec() {
        let mut number_vec = vec![4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
        cocktail_shaker_sort(&mut number_vec);
        assert_eq!(number_vec, vec![-31, 0, 1, 2, 2, 4, 65, 83, 99, 782]);
    }

    #[test]
    fn basic() {
        let mut basic = vec![11, 8, 21, 5, 10, 3, 13, 2, 34];
        cocktail_shaker_sort(&mut basic);
        assert!(is_sorted(&basic));
    }

    #[test]
    fn test_string_vec() {
        let mut string_vec = vec!["beach", "hotel", "airplane", "car", "house", "art"];
        cocktail_shaker_sort(&mut string_vec);
        assert_eq!(
            string_vec,
            vec!["airplane", "art", "beach", "car", "hotel", "house"]
        );
    }
}
