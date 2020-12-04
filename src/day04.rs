use std::collections::BTreeMap;
#[derive(Debug)]
pub struct Passport(BTreeMap<String, String>);

impl Passport {
    fn is_valid(&self) -> bool {
        ["byr", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"]
            .iter()
            .all(|&key| self.0.contains_key(key))
    }

    fn is_valid_values(&self) -> bool {
        self.is_valid()
            && self.is_valid_range("byr", 1920, 2002)
            && self.is_valid_range("iyr", 2010, 2020)
            && self.is_valid_range("eyr", 2020, 2030)
            && self.is_valid_height()
            && self.is_valid_haircolor()
            && self.is_valid_eyecolor()
            && self.is_valid_pid()
    }

    fn is_valid_range(&self, key: &str, lower: usize, upper: usize) -> bool {
        let value = self.0[key].parse().unwrap_or(0);
        lower <= value && value <= upper
    }

    fn is_valid_height(&self) -> bool {
        let height = self.0["hgt"].as_str();
        let value = height[..height.len() - 2].parse().unwrap_or(0);

        if height.ends_with("cm") {
            150 <= value && value <= 193
        } else {
            59 <= value && value <= 76
        }
    }

    fn is_valid_haircolor(&self) -> bool {
        let color = self.0["hcl"].as_str();

        color.len() == 7 && {
            let mut iter = color.bytes();
            iter.next() == Some(b'#') && iter.all(|x| x.is_ascii_hexdigit())
        }
    }

    fn is_valid_eyecolor(&self) -> bool {
        let color = self.0["ecl"].as_str();

        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&color)
    }

    fn is_valid_pid(&self) -> bool {
        let pid = self.0["pid"].as_str();

        pid.len() == 9 && pid.bytes().all(|x| x.is_ascii_digit())
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
        generator(SAMPLE);
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE).unwrap()), 2);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(VALID).unwrap()), 4);

        assert_eq!(part2(&generator(INVALID).unwrap()), 0);
    }
}
