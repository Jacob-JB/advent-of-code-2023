
const INPUT: &str = include_str!("input.txt");

const WIDTH: usize = 140;
const HEIGHT: usize = 140;

// const INPUT: &str = include_str!("test_input.txt");

// const WIDTH: usize = 10;
// const HEIGHT: usize = 10;

#[derive(Debug, Clone, Copy)]
enum Character {
    Digit(char),
    Part,
    Gear {
        adjacent_parts: u32,
        ratio: u32,
    },
    Period,
}

impl From<char> for Character {
    fn from(value: char) -> Self {
        match value {
            _ if value.is_ascii_digit() => Character::Digit(value),
            '*' => Character::Gear {
                adjacent_parts: 0,
                ratio: 1,
            },
            '.' => Character::Period,
            _ => Character::Part,
        }
    }
}

impl Character {
    fn is_digit(self) -> bool {
        if let Character::Digit(_) = self {
            true
        } else {
            false
        }
    }
}

fn main() {
    let mut grid: [[Character; WIDTH]; HEIGHT] = INPUT.split('\n').map(
        |line| line.trim().chars().map(char::into).collect::<Vec<_>>().try_into().unwrap()
    ).collect::<Vec<_>>().try_into().unwrap();

    let mut current_number = String::new();

    let mut part_sum = 0;

    for (y, line) in grid.clone().iter().enumerate() {
        for (x, character) in line.into_iter().enumerate() {
            if let &Character::Digit(digit) = character {
                current_number.push(digit);

                // check if this is the last digit of the number

                if grid[y].get(x + 1).map(|next_character| !next_character.is_digit()).unwrap_or(true) {

                    // number has finished

                    let left_x = x + 1 - current_number.len();
                    let right_x = x;

                    let part_number = current_number.as_str().parse::<u32>().unwrap();

                    fn check(character: &mut Character, part_number: u32) -> bool {
                        if let Character::Gear { adjacent_parts, ratio } = character {
                            *adjacent_parts += 1;
                            *ratio *= part_number;
                            true
                        } else if let Character::Part = character {
                            true
                        } else {
                            false
                        }
                    }

                    // check left
                    let is_part_number = (
                        left_x > 0 && check(&mut grid[y][left_x - 1], part_number)
                    ) ||
                    // check right
                    (
                        right_x + 1 < WIDTH && check(&mut grid[y][right_x + 1], part_number)
                    ) ||
                    // check above and below
                    (
                        (left_x.saturating_sub(1)..=(right_x + 1).min(WIDTH - 1)).any(
                            |x|
                            (y > 0 && check(&mut grid[y - 1][x], part_number)) ||
                            (y + 1 < HEIGHT && check(&mut grid[y + 1][x], part_number))
                        )
                    );

                    if is_part_number {
                        part_sum += part_number;
                    }

                    dbg!((current_number, is_part_number));

                    current_number = String::new();
                }
            }
        }
    }

    let mut ratio_sum = 0;

    for row in &grid {
        for character in row {
            if let Character::Gear { adjacent_parts: 2, ratio } = character {
                ratio_sum += ratio;
            }
        }
    }

    dbg!(part_sum);
    dbg!(ratio_sum);
}
