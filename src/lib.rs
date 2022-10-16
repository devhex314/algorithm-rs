mod search;
mod sorting;

#[cfg(test)]
mod tests {

    use super::*;
    use sorting::bubble_sort;
    #[test]
    fn test_bubble() {
        let mut num = [4, 3, 2, 1];
        bubble_sort(&mut num);
        assert_eq!(num, [1, 2, 3, 4])
    }

    use search::binary_search;
    #[test]
    fn test_binary_search() {
        let test_arr = [1, 5, 6, 7, 22, 46, 76, 88, 96, 100];
        for i in 0..test_arr.len() {
            assert_eq!(i, binary_search(&test_arr, test_arr[i]).unwrap())
        }
    }
}
