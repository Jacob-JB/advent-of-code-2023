
// const INPUT: &str = include_str!("test_input.txt");
const INPUT: &str = include_str!("input.txt");


struct Card {
    winning_numbers: Vec<u32>,
    owned_numbers: Vec<u32>,
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let mut parts = value.split(" | ");

        Card {
            winning_numbers: parts.next().unwrap().split(' ')
            .filter(|string| !string.is_empty())
            .map(|number| number.parse().unwrap())
            .collect(),

            owned_numbers: parts.next().unwrap().split(' ')
            .filter(|string| !string.is_empty())
            .map(|number| number.parse().unwrap())
            .collect(),
        }
    }
}

impl Card {
    fn matches(&self) -> u32 {
        self.owned_numbers.iter().filter(
            |number| self.winning_numbers.contains(&number)
        ).count() as u32
    }

    fn points(&self) -> u32 {
        let matches = self.matches();

        if matches > 0 {
            2u32.pow(matches as u32 - 1)
        } else {
            0
        }
    }
}


fn main() {
    let cards: Vec<Card> = INPUT.split('\n').map(|line| {
        let mut parts = line.trim().split(": ");

        let card: Card = parts.nth(1).unwrap().into();

        card
    }).collect();


    let part_a_sum: u32 = cards.iter().map(Card::points).sum();

    dbg!(part_a_sum);


    fn card_value(cards: &[Card], card_number: usize) -> u32 {
        let matches = cards.get(card_number).unwrap().matches() as usize;

        1 +
        (0..matches)
        .map(|n| card_value(cards, n + card_number + 1))
        .sum::<u32>()
    }

    let part_b_sum: u32 = (0..cards.len()).map(|n| card_value(&cards, n)).sum();

    dbg!(part_b_sum);
}
