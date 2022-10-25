use std::cmp;

pub fn pancake_sort<T>(arr: &mut [T]) -> Vec<T>
where
    T: cmp::PartialEq + cmp::Ord + cmp::PartialOrd + Clone,
{
    let len = arr.len();
    if len < 2 {
        arr.to_vec();
    }
    for i in (0..len).rev() {
        let max_index = arr
            .iter()
            .take(i + 1)
            .enumerate()
            .max_by_key(|&(_, elem)| elem)
            .map(|(idx, _)| idx)
            .unwrap();
        if max_index != i {
            arr[0..max_index + 1].reverse();
            arr[0..i + 1].reverse();
        }
    }
    arr.to_vec()
}
