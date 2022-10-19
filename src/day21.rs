use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq)]
pub struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

#[aoc_generator(day21)]
pub fn generator(input: &str) -> Vec<Food> {
    input
        .lines()
        .map(|line| {
            let mut parens = line.split(&['(', ')'][..]);

            let ingredients = parens
                .next()
                .unwrap()
                .split(' ')
                .filter_map(|x| {
                    if !x.is_empty() {
                        Some(x.to_string())
                    } else {
                        None
                    }
                })
                .collect();

            let allergens = parens
                .next()
                .unwrap()
                .trim_start_matches("contains")
                .split(", ")
                .map(|x| x.trim())
                .filter_map(|x| {
                    if !x.is_empty() {
                        Some(x.to_string())
                    } else {
                        None
                    }
                })
                .collect();

            Food {
                ingredients,
                allergens,
            }
        })
        .collect()
}

fn solve(foods: &[Food]) -> (HashMap<&str, HashSet<&str>>, HashMap<&str, usize>) {
    let all_allergens = foods
        .iter()
        .flat_map(|food| food.allergens.iter().map(|x| x.as_str()))
        .collect::<HashSet<_>>();

    // Rather than try to find unique allergens, all we can do is remove impossible allergens
    // we start with a HashMap mapping the ingredient to all the possible allergens.
    let mut possibilities = foods
        .iter()
        .flat_map(|food| food.ingredients.iter().map(|x| x.as_str()))
        .map(|ing| (ing, all_allergens.clone()))
        .collect::<HashMap<_, _>>();

    let mut frequency = HashMap::new();
    for food in foods.iter() {
        for ing in food.ingredients.iter() {
            *frequency.entry(ing.as_str()).or_insert(0) += 1;
        }

        // For every food, we remove possible allergens for their ingredients.
        // e.g. All ingredients set: {fvjkl, kfcds, mxmxvkd, nhms, sbzzf, sqjhc, trh}
        //      The first food entry: mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
        //
        //      This means that complement ingredients is {fvjkl, sbzzf, trh}
        //      These ingredients are not {dairy, fish}, so we remove them from the possiblities
        //
        //      Next food is: trh fvjkl sbzzf mxmxvkd (contains dairy)
        //      the complement is {kfcds, nhms, sqjhc} and we can remove diary
        //
        //      Repeat for all foods.
        //
        //      This yields us this possiblity map:
        //      {
        //          "fvjkl": {"soy"},
        //          "kfcds": {},
        //          "mxmxvkd": {"dairy", "fish"},
        //          "nhms": {},
        //          "sbzzf": {},
        //          "sqjhc": {"fish", "soy"}
        //          "trh": {},
        //      }
        //
        //      The ingredients with no allergies, solve part1
        for allergen in food.allergens.iter().map(|f| f.as_str()) {
            for (&ing, s) in possibilities.iter_mut() {
                if !food.ingredients.contains(ing) {
                    s.remove(allergen);
                }
            }
        }
    }

    (possibilities, frequency)
}

#[aoc(day21, part1)]
pub fn part1(foods: &[Food]) -> usize {
    let (possibilities, frequency) = solve(foods);
    // println!("{:?}", possibilities);

    possibilities
        .iter()
        .filter_map(|(ing, allergens)| {
            if allergens.is_empty() {
                Some(frequency[*ing])
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day21, part2)]
pub fn part2(foods: &[Food]) -> String {
    let (mut possibilities, _) = solve(foods);
    possibilities.retain(|_, s| !s.is_empty());

    // let mut allergens = BTreeSet::new();
    let mut allergens = Vec::new();

    while let Some((allergen, ingredient)) = possibilities.iter().find_map(|(&ing, s)| {
        if s.len() == 1 {
            s.iter().next().copied().map(|x| (x, ing))
        } else {
            None
        }
    }) {
        allergens.push((allergen, ingredient));

        for s in possibilities.values_mut() {
            s.remove(allergen);
        }
    }

    // println!("{:?}", allergens);
    allergens.sort_unstable();

    allergens
        .iter()
        .map(|&(_, ing)| ing)
        .enumerate()
        .fold(String::new(), |mut acc, (i, ing)| {
            if i != 0 {
                acc.push(',');
            }

            acc.push_str(ing);
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    #[ignore]
    pub fn test_input() {
        println!("{:?}", generator(SAMPLE));

        // assert_eq!(generator(SAMPLE), Object());
    }

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 5);
    }

    #[test]
    pub fn test2() {
        assert_eq!(&part2(&generator(SAMPLE)), "mxmxvkd,sqjhc,fvjkl");
    }

    mod regression {
        use super::*;

        const INPUT: &str = include_str!("../input/2020/day21.txt");
        const ANSWERS: (usize, &str) = (2230, "qqskn,ccvnlbp,tcm,jnqcd,qjqb,xjqd,xhzr,cjxv");

        #[test]
        pub fn test() {
            let input = INPUT.trim_end_matches('\n'); // Trims trailing newline

            assert_eq!(part1(&generator(input)), ANSWERS.0);
            assert_eq!(&part2(&generator(input)), ANSWERS.1);
        }
    }
}
