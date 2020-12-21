use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut allergen_potential_ingredients = HashMap::new();
    let mut ingredient_potential_allergens = HashMap::new();
    let mut ingredient_count: HashMap<_, usize> = HashMap::new();

    for line in buffer.lines() {
        let mut groups = line.split(|c| c == '(' || c == ')');
        match (groups.next(), groups.next()) {
            (Some(ingredients_list), Some(allergens_list))
                if allergens_list.starts_with("contains ") =>
            {
                let ingredients = ingredients_list
                    .trim()
                    .split_ascii_whitespace()
                    .collect::<HashSet<_>>();
                let allergens = allergens_list["contains ".len()..]
                    .split(", ")
                    .collect::<HashSet<_>>();
                for ingredient in ingredients.clone() {
                    *ingredient_count.entry(ingredient).or_insert(0) += 1;

                    let allergen_set = ingredient_potential_allergens
                        .entry(ingredient)
                        .or_insert(allergens.clone());
                    *allergen_set = allergen_set
                        .union(&allergens)
                        .map(|s| *s)
                        .collect::<HashSet<_>>();
                }
                for allergen in allergens {
                    let ingredient_set = allergen_potential_ingredients
                        .entry(allergen)
                        .or_insert(ingredients.clone());
                    *ingredient_set = ingredient_set
                        .intersection(&ingredients)
                        .map(|s| *s)
                        .collect::<HashSet<_>>();
                }
            }
            _ => continue,
        }
    }

    let mut ones = allergen_potential_ingredients
        .iter()
        .filter(|(_, s)| s.len() == 1)
        .map(|(s, _)| *s)
        .collect::<Vec<&str>>();

    let mut ingredient_allergen_names = HashMap::new();
    while !ones.is_empty() {
        let allergen = ones.pop().unwrap();

        let ingredient_one_set = allergen_potential_ingredients.get(allergen).unwrap().clone();

        let ingredient = *ingredient_one_set.iter().next().unwrap();

        allergen_potential_ingredients.remove(allergen);
        ingredient_allergen_names.insert(ingredient, allergen);

        let allergen_set = ingredient_potential_allergens.get(ingredient).unwrap();

        for allerg in allergen_set {
            if let Some(ingredient_set) = allergen_potential_ingredients.get_mut(*allerg) {
                ingredient_set.remove(ingredient);
                if ingredient_set.len() == 1 {
                    ones.push(allerg);
                }
            }
        }
    }

    let mut part_1: usize = ingredient_count.values().sum();
    for ingredient in ingredient_allergen_names.keys() {
        part_1 -= ingredient_count.get(ingredient).unwrap();
    }

    println!("{}", part_1);

    let mut part_2 = ingredient_allergen_names.keys().map(|k| *k).collect::<Vec<_>>();
    part_2.sort_by(|a, b| ingredient_allergen_names.get(a).unwrap().cmp(ingredient_allergen_names.get(b).unwrap()));

    println!("{}", part_2.join(","));

    Ok(())
}
