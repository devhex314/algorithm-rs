pub fn radix_sort(arr: &mut [u64]) {
    let max: usize = match arr.iter().max() {
        Some(&x) => x as usize,
        None => return,
    };
    // Make radix a power of 2 close to arr.len() for optimal runtime
    let radix = arr.len().next_power_of_two();
    // Counting sort by each digit from least to most significant
    let mut place = 1;
    while place <= max {
        let digit_of = |x| x as usize / place % radix;
        // Count digit occurrences
        let mut counter = vec![0; radix];
        for &x in arr.iter() {
            counter[digit_of(x)] += 1;
        }
        // Compute last index of each digit
        for i in 1..radix {
            counter[i] += counter[i - 1];
        }
        // Write elements to their new indices
        for &x in arr.to_owned().iter().rev() {
            counter[digit_of(x)] -= 1;
            arr[counter[digit_of(x)]] = x;
        }
        place *= radix;
    }
}
