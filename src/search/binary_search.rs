pub fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() as i32 - 1;
    while low <= high {
        let mid = (high + low) / 2;
        let m_index = mid as usize;
        let val = &arr[m_index];
        if *val == target {
            return Some(m_index);
        }
        if *val < target {
            low = mid + 1
        }
        if *val > target {
            high = mid - 1
        }
    }
    None
}
