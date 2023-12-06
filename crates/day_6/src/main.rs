
const INPUT: &str = include_str!("input.txt");
// const INPUT: &str = include_str!("test_input.txt");

fn main() {
    let mut parts = INPUT.split("\r\n")
    .map(
        |line| line.split(' ').filter_map(|e| e.parse::<f64>().ok()).collect::<Vec<_>>()
    );

    let times = parts.next().unwrap();
    let distances = parts.next().unwrap();

    let product: u64 =  times.iter().zip(distances.iter())
    .map(|(time, distance)| ways_to_win(*time, *distance))
    .product();

    dbg!(product);


    let mut parts = INPUT.split("\r\n")
    .map(
        |line| line.chars().filter(char::is_ascii_digit).collect::<String>()
    );

    let big_time = parts.next().unwrap().parse::<f64>().unwrap();
    let big_distance = parts.next().unwrap().parse::<f64>().unwrap();

    dbg!(big_time);
    dbg!(big_distance);

    dbg!(ways_to_win(big_time, big_distance));
}

fn ways_to_win(time: f64, distance: f64) -> u64 {
    dbg!((time, distance));

        let root = (time * time - 4. * distance).sqrt();

        let lower = (time - root) / 2.;
        let upper = (time + root) / 2.;

        let diff = dbg!(dbg!(upper).floor() as u64) - dbg!(dbg!(lower).ceil() as u64);
        let mut ways = dbg!(diff + 1);

        if lower % 1.0 == 0. {
            ways -= 2;
        }

        dbg!(ways)
}
