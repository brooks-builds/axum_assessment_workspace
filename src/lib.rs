use doubler_crate::double;
use sum_crate::sum;

pub fn run(number: i32, number2: i32) -> i32 {
    let doubled = double(number);
    sum(doubled, number2)
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn run_doubles_and_sums() {
        let number1 = 10;
        let number2 = 20;
        let expected = 40;

        assert_eq!(run(number1, number2), expected);
    }
}
