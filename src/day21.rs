use std::collections::{BTreeSet, HashMap, HashSet};

#[derive(Debug, PartialEq)]
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
                .filter(|x| !x.is_empty())
                .map(|x| x.to_string())
                .collect();

            let allergens = parens
                .next()
                .unwrap()
                .trim_start_matches("contains")
                .split(", ")
                .map(|x| x.trim())
                .filter(|x| !x.is_empty())
                .map(|x| x.to_string())
                .collect();

            Food {
                ingredients,
                allergens,
            }
        })
        .collect()
}

fn solve(foods: &[Food]) -> (HashMap<&str, HashSet<&str>>, HashMap<&str, usize>) {
    let all_ingredients = foods
        .iter()
        .flat_map(|food| food.ingredients.iter().map(|x| x.as_str()))
        .collect::<HashSet<_>>();

    let all_allergens = foods
        .iter()
        .flat_map(|food| food.allergens.iter().map(|x| x.as_str()))
        .collect::<HashSet<_>>();

    let mut possibilities = all_ingredients
        .iter()
        .map(|i| (*i, all_allergens.clone()))
        .collect::<HashMap<_, _>>();

    let mut frequency = HashMap::new();
    for food in foods.iter() {
        for ing in food.ingredients.iter() {
            *frequency.entry(ing.as_str()).or_insert(0) += 1;
        }

        // For every ingredient, check against every food.
        // if the food does not contain the ingredient, then we can remove it
        // as a possible allergen
        for &ing in all_ingredients.iter() {
            for allergens in food.allergens.iter().map(|f| f.as_str()) {
                if !food.ingredients.contains(ing) {
                    possibilities.entry(ing).and_modify(|s| {
                        s.remove(allergens);
                    });
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
        .map(|(ing, allergens)| {
            if allergens.is_empty() {
                frequency[*ing]
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day21, part2)]
pub fn part2(foods: &[Food]) -> String {
    let (mut possibilities, _) = solve(foods);
    let mut allergens = BTreeSet::new();

    loop {
        // println!("{:?}", possibilities);
        let mut allergen = None;
        for (&ing, s) in possibilities.iter() {
            if s.len() == 1 {
                allergen = s.iter().next().copied();
                if let Some(allergen) = allergen {
                    allergens.insert((allergen, ing));
                }
            }
        }

        if let Some(allergens) = allergen {
            for s in possibilities.values_mut() {
                s.remove(allergens);
            }
        }

        possibilities.retain(|_, s| !s.is_empty());
        if possibilities.is_empty() {
            break;
        }
    }

    // println!("{:?}", allergens);

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
