use std::{
    collections::{BTreeMap, HashMap, HashSet},
    io::{self, BufRead},
    str::FromStr,
};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use simple_error::SimpleError;

#[derive(Clone)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl FromStr for Food {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref FOOD_RE: Regex = Regex::new(r"(.*) \(contains (.*)\)").unwrap();
        }

        if let Some(caps) = FOOD_RE.captures(s) {
            Ok(Food {
                ingredients: caps[1].split(' ').map(|s| s.to_owned()).collect(),
                allergens: caps[2].split(", ").map(|s| s.to_owned()).collect(),
            })
        } else {
            Err(SimpleError::new("food parse fiasco"))
        }
    }
}

fn main() {
    let foods = read_food(io::stdin().lock());
    println!("Day 21, part 1: {}", part1(&foods));
    println!("Day 21, part 2: {}", part2(&foods));
}

fn allergens_to_possible_ingredients(foods: &[Food]) -> HashMap<String, HashSet<String>> {
    let mut allergens_to_possible_ingredients: HashMap<String, HashSet<String>> = HashMap::new();
    for food in foods.iter() {
        for allergen in food.allergens.iter() {
            if let Some(possible_ingredients) = allergens_to_possible_ingredients.get_mut(allergen)
            {
                *possible_ingredients = possible_ingredients
                    .intersection(&food.ingredients)
                    .cloned()
                    .collect();
            } else {
                allergens_to_possible_ingredients
                    .insert(allergen.to_owned(), food.ingredients.clone());
            }
        }
    }

    allergens_to_possible_ingredients
}

fn part1(foods: &[Food]) -> usize {
    let allergens_to_possible_ingredients = allergens_to_possible_ingredients(foods);

    let mut allergens: HashSet<String> = HashSet::new();
    for (_, possible_ingredients) in allergens_to_possible_ingredients.iter() {
        allergens = allergens.union(possible_ingredients).cloned().collect();
    }

    let all_ingredients = foods.iter().fold(HashSet::new(), |all, food| {
        all.union(&food.ingredients).cloned().collect()
    });
    let safe: HashSet<String> = all_ingredients.difference(&allergens).cloned().collect();
    foods.iter().fold(0, |sum, food| {
        sum + safe.intersection(&food.ingredients).count()
    })
}
fn part2(foods: &[Food]) -> String {
    let mut foods = foods.to_vec();
    let all_ingredients = foods.iter().fold(HashSet::new(), |all, food| {
        all.union(&food.ingredients).cloned().collect()
    });

    let allergens_to_possible_ingredients = allergens_to_possible_ingredients(&foods);
    let mut ingredients_allergens: HashMap<String, String> = HashMap::new();

    let mut found_any = true;
    while found_any {
        found_any = false;

        // Find an allergen
        for (allergen, _) in allergens_to_possible_ingredients.iter() {
            let mut ingredients = all_ingredients.clone();
            for food in foods.iter() {
                if food.allergens.contains(allergen) {
                    ingredients = ingredients
                        .intersection(&food.ingredients)
                        .cloned()
                        .collect();
                }
            }

            if ingredients.len() == 1 {
                let ingredient = ingredients.iter().next().unwrap();
                ingredients_allergens.insert(ingredient.clone(), allergen.clone());
                found_any = true;
                break;
            }
        }

        // Update the food list
        for food in foods.iter_mut() {
            for (ingredient, _) in ingredients_allergens.iter() {
                let to_set = |ingredient: &str| {
                    let mut result: HashSet<String> = HashSet::new();
                    result.insert(ingredient.to_owned());
                    result
                };

                food.ingredients = food
                    .ingredients
                    .difference(&to_set(ingredient))
                    .cloned()
                    .collect();
                food.allergens = food
                    .allergens
                    .difference(&to_set(&ingredients_allergens[ingredient]))
                    .cloned()
                    .collect();
            }
        }
    }

    let mut result: BTreeMap<String, String> = BTreeMap::new();
    for (k, v) in ingredients_allergens.iter() {
        result.insert(v.clone(), k.clone());
    }
    result.values().join(",")
}

fn read_food<R: BufRead>(reader: R) -> Vec<Food> {
    reader
        .lines()
        .map(|l| Food::from_str(&l.unwrap()).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_solution() {
        let foods = read_food(BufReader::new(File::open("inputs/day21/1.txt").unwrap()));
        assert_eq!(part1(&foods), 2874);
        assert_eq!(
            part2(&foods),
            "gfvrr,ndkkq,jxcxh,bthjz,sgzr,mbkbn,pkkg,mjbtz"
        );
    }
}
