
const INPUT: &str = include_str!("input.txt");

const DIGIT_PATTERNS: [(&str, u32); 18] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),

    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

fn main() {
    let mut sum = 0;

    for line in INPUT.split('\n') {

        let mut first_digit = None;
        let mut last_digit = None;

        for &(pattern, digit) in DIGIT_PATTERNS.iter() {

            // all occurences of the digit in that line
            for (position, _) in line.match_indices(pattern) {
                if first_digit.map(|(best_position, _)| position < best_position).unwrap_or(true) {
                    first_digit = Some((position, digit));
                }

                if last_digit.map(|(best_position, _)| position > best_position).unwrap_or(true) {
                    last_digit = Some((position, digit));
                }
            }
        }

        let (_, first_digit) = first_digit.expect(&format!("no first digit on line {}", line));
        let (_, last_digit) = last_digit.expect(&format!("no last digit on line {}", line));

        sum += first_digit * 10 + last_digit;
    }

    dbg!(sum);
}
