use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub struct Passport(BTreeMap<String, String>);

impl Passport {
    fn is_valid(&self) -> bool {
        self.0
            .keys()
            .filter(|&key| key != "cid")
            .eq(["byr", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"].iter())
    }

    fn is_valid_values(&self) -> bool {
        self.is_valid()
            && self
                .0
                .iter()
                .all(|(key, value)| Self::validate_value(key, value))
    }

    fn validate_value(key: &str, value: &str) -> bool {
        match key {
            "byr" => Self::is_valid_range(value, 1920, 2002),
            "iyr" => Self::is_valid_range(value, 2010, 2020),
            "eyr" => Self::is_valid_range(value, 2020, 2030),
            "hgt" => Self::is_valid_height(value),
            "hcl" => Self::is_valid_haircolor(value),
            "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value),
            "pid" => value.len() == 9 && value.bytes().all(|x| x.is_ascii_digit()),
            "cid" => true,
            _ => false,
        }
    }

    fn is_valid_range(s: &str, lower: usize, upper: usize) -> bool {
        s.parse()
            .map(|value| lower <= value && value <= upper)
            .unwrap_or(false)
    }

    fn is_valid_height(height: &str) -> bool {
        if let Some(centimeter) = height.strip_suffix("cm") {
            Self::is_valid_range(centimeter, 150, 193)
        } else if let Some(inches) = height.strip_suffix("in") {
            Self::is_valid_range(inches, 59, 76)
        } else {
            false
        }
    }

    fn is_valid_haircolor(color: &str) -> bool {
        color.len() == 7 && {
            let mut iter = color.bytes();
            iter.next() == Some(b'#') && iter.all(|x| x.is_ascii_hexdigit())
        }
    }
}

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Option<Vec<Passport>> {
    input
        .split("\n\n")
        .map(|section| {
            section
                .split(&[' ', '\n'][..])
                .map(|field| {
                    let mut iter = field.split(':');

                    iter.next()
                        .map(|s| s.to_owned())
                        .and_then(|first| iter.next().map(|s| (first, s.to_owned())))
                })
                .collect::<Option<_>>()
                .map(Passport)
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(inputs: &[Passport]) -> usize {
    inputs.iter().filter(|&pass| pass.is_valid()).count()
}

#[aoc(day4, part2)]
pub fn part2(inputs: &[Passport]) -> usize {
    inputs.iter().filter(|&pass| pass.is_valid_values()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    const VALID: &str = r"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    const INVALID: &str = r"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    #[test]
    pub fn test_input() {
        assert_eq!(
            generator(SAMPLE)
                .unwrap()
                .iter()
                .map(|passport| passport
                    .0
                    .iter()
                    .map(|(s, t)| (s.as_str(), t.as_str()))
                    .collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            vec![
                vec![
                    ("byr", "1937"),
                    ("cid", "147"),
                    ("ecl", "gry"),
                    ("eyr", "2020"),
                    ("hcl", "#fffffd"),
                    ("hgt", "183cm"),
                    ("iyr", "2017"),
                    ("pid", "860033327")
                ],
                vec![
                    ("byr", "1929"),
                    ("cid", "350"),
                    ("ecl", "amb"),
                    ("eyr", "2023"),
                    ("hcl", "#cfa07d"),
                    ("iyr", "2013"),
                    ("pid", "028048884")
                ],
                vec![
                    ("byr", "1931"),
                    ("ecl", "brn"),
                    ("eyr", "2024"),
                    ("hcl", "#ae17e1"),
                    ("hgt", "179cm"),
                    ("iyr", "2013"),
                    ("pid", "760753108")
                ],
                vec![
                    ("ecl", "brn"),
                    ("eyr", "2025"),
                    ("hcl", "#cfa07d"),
                    ("hgt", "59in"),
                    ("iyr", "2011"),
                    ("pid", "166559648")
                ]
            ]
        );
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE).unwrap()), 2);
    }

    #[test]
    pub fn test2_valid() {
        assert_eq!(part2(&generator(VALID).unwrap()), 4);
    }

    #[test]
    pub fn test2_invalid() {
        assert_eq!(part2(&generator(INVALID).unwrap()), 0);
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2020/day4.txt");
        const ANSWERS: (usize, usize) = (192, 101);

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline
            assert_eq!(part1(&generator(input).unwrap()), ANSWERS.0);
            assert_eq!(part2(&generator(input).unwrap()), ANSWERS.1);
        }
    }
}
