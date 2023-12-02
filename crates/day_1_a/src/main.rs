
const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut sum = 0;

    let mut first_digit: Option<u32> = None;
    let mut last_digit: Option<u32> = None;

    for char in INPUT.chars() {
        if let Ok(digit) = char.to_string().parse() {
            if first_digit.is_none() {
                first_digit = Some(digit);
            }

            last_digit = Some(digit);
        }

        else if let '\n' = char {
            if let (Some(first_digit), Some(last_digit)) = (first_digit, last_digit) {
                sum += dbg!(first_digit * 10 + last_digit);

            }

            first_digit = None;
            last_digit = None;
        }
    }

    dbg!(sum);
}
