
mod sorting;

#[cfg(test)]
mod tests {

    use super::*;
    use sorting::bubble::*;
    #[test]
    fn test_bubble() {
        let mut num = [4,3,2,1];
        bubble_sort(&mut num);
        assert_eq!(num, [1,2,3,4])
    }
}
