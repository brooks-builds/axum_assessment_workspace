pub fn double(number: i32) -> i32 {
    number * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubles_input() {
        let number = 15;
        let expected = 30;
        assert_eq!(double(number), expected);
    }
}
