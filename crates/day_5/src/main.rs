use std::{str::FromStr, convert::Infallible};

const MAPS: &str = include_str!("maps.txt");
const SEEDS: &[i64] = &[
    3489262449,
    222250568,

    2315397239,
    327729713,

    1284963,
    12560465,

    1219676803,
    10003052,

    291763704,
    177898461,

    136674754,
    107182783,

    2917625223,
    260345082,

    1554280164,
    216251358,

    3900312676,
    5629667,

    494259693,
    397354410,
];

// const MAPS: &str = include_str!("test_input.txt");
// const SEEDS: &[i64] = &[
//     79,
//     14,
//     55,
//     13,
// ];

#[derive(Debug)]
struct RangeConversion {
    source: i64,
    size: i64,

    dest: i64,
}

#[derive(Debug)]
struct Map {
    conversions: Vec<RangeConversion>,
}


impl FromStr for RangeConversion {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');

        Ok(RangeConversion {
            dest: parts.next().unwrap().parse().unwrap(),
            source: parts.next().unwrap().parse().unwrap(),
            size: parts.next().unwrap().parse().unwrap(),
        })
    }
}

impl FromStr for Map {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Map {
            conversions: s.split("\r\n").map(
                |line| line.parse().unwrap()
            ).collect()
        })
    }
}

impl RangeConversion {
    /// if the number falls within the range it will get mapped
    fn try_convert(&self, number: i64) -> Option<i64> {
        if number >= self.source && number < (self.source + self.size) {
            Some(number + (self.dest - self.source))
        } else {
            None
        }
    }


    /// will map out of `ranges`,
    /// what is left behind is what wasn't mapped,
    /// it should be recombined for a proper map
    fn convert_ranges(&self, ranges: &mut Ranges) -> Ranges {
        // we need to know what ranges we map and what stays the same
        let mut result_ranges = Vec::new();
        let mut remaining_ranges = Vec::new();

        let self_start = self.source;
        let self_end = self.source + self.size - 1;

        for &(input_start, input_size) in ranges.ranges.iter() {
            let input_end = input_start + input_size - 1;
            // does it intercept

            // input is entirely outside the range
            if input_end < self_start || self_end < input_start {
                remaining_ranges.push((
                    input_start,
                    input_size
                ));

                continue;
            }

            // must be intersecting at this point

            let result_start = input_start.max(self_start);
            let result_end = input_end.min(self_end);
            let result_size = result_end - result_start + 1;

            result_ranges.push((
                result_start + (self.dest - self.source),
                result_size,
            ));

            // find what ranges are remaining

            // lower part
            if input_start < self_start {
                remaining_ranges.push((
                    input_start,
                    (self_start - input_start),
                ));
            }

            // upper part
            if input_end > self_end {
                remaining_ranges.push((
                    self_end + 1,
                    (input_end - (self_end + 1)) + 1,
                ));
            }
        }

        *ranges = Ranges { ranges: remaining_ranges };
        Ranges { ranges: result_ranges }
    }
}

impl Map {
    fn convert(&self, number: i64) -> i64 {
        for convertion in self.conversions.iter() {
            if let Some(result) = convertion.try_convert(number) {
                return result;
            }
        }

        number
    }

    fn convert_ranges(&self, mut ranges: Ranges) -> Ranges {
        let mut result_ranges = Ranges {
            ranges: Vec::new(),
        };

        for conversion in self.conversions.iter() {
            let mapped = conversion.convert_ranges(&mut ranges);

            for (start, size) in mapped.ranges {
                result_ranges.merge(start, size);
            }
        }

        for (start, size) in ranges.ranges {
            result_ranges.merge(start, size);
        }

        result_ranges
    }
}

#[derive(Debug)]
struct Ranges {
    /// a list of starts and sizes
    ranges: Vec<(i64, i64)>,
}

impl Ranges {
    fn merge(&mut self, start: i64, size: i64) {
        let input_start = start;
        let input_end = start + size - 1;

        let mut intercepting_ranges = vec![(start, input_end)];

        // extract all the rangtes that intercept the input
        self.ranges.retain(
            |&(start, size)| {
                let end = start + size - 1;

                if input_end < start {
                    return true;
                }

                if end < input_start {
                    return true;
                }

                intercepting_ranges.push((start, end));

                false
            }
        );

        // find the merged start
        let merged_start = intercepting_ranges.iter().map(|&(start, _)| start).min().unwrap();
        let merged_end = intercepting_ranges.iter().map(|&(_, end)| end).max().unwrap();

        self.ranges.push((
            merged_start,
            (merged_end - merged_start) + 1,
        ));
    }
}


fn main() {
    let maps: Vec<Map> =
    MAPS.split("\r\n\r\n")
    .map(|map| {
        let mut parts = map.split(":\r\n");

        let name = parts.next().unwrap();
        // dbg!(name);
        parts.next().unwrap().parse().unwrap()
    })
    .collect();

    let mut smallest = i64::MAX;

    for &seed in SEEDS {
        let mut previous = seed;

        for map in maps.iter() {
            previous = map.convert(previous);
        }

        smallest = smallest.min(previous);
    }

    dbg!(smallest);

    // dbg!(RangeConversion {
    //     source: 5,
    //     size: 4,
    //     dest: 15,
    // }.convert_ranges(&Ranges {
    //     ranges: vec![(4, 4), (1, 1)],
    // }));

    let mut smallest = i64::MAX;

    let mut iter = SEEDS.iter();
    let mut i = 0;

    while let (Some(&start), Some(&size)) = (iter.next(), iter.next()) {
        i = dbg!(i) + 1;
        let mut previous = Ranges { ranges: vec![(start, size)] };

        for (mi, map) in maps.iter().enumerate() {
            dbg!(mi);
            previous = dbg!(map.convert_ranges(previous));
        }

        smallest = smallest.min(
            previous.ranges.iter().map(|&(s, _)| s).min().unwrap()
        );
    }

    dbg!(smallest);

    // let mut x = Ranges {
    //     ranges: vec![
    //         (1, 5),
    //         (11, 5),
    //         (21, 10),
    //     ],
    // };

    // x.merge(5, 7);

    // dbg!(x);

    // let start = Ranges {
    //     ranges: vec![(55, 15)],
    // };

    // let map = Map {
    //     conversions: vec![
    //         RangeConversion {
    //             source: 53,
    //             size: 8,
    //             dest: 49,
    //         },
    //         RangeConversion {
    //             source: 11,
    //             size: 0,
    //             dest: 42,
    //         },
    //         RangeConversion {
    //             source: 0,
    //             size: 42,
    //             dest: 7,
    //         },
    //         RangeConversion {
    //             source: 7,
    //             size: 57,
    //             dest: 4,
    //         },
    //     ],
    // };

    // dbg!(map.convert_ranges(start));
}
