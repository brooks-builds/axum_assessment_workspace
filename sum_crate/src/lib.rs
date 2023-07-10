pub fn sum(left: i32, right: i32) -> i32 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_sums_two_numbers_together() {
        let one = 10;
        let two = 20;
        let expected = 30;
        assert_eq!(sum(one, two), expected);
    }
}
