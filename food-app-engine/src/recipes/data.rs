// this is where we will load the data
use super::structs::ingredient_type::IngredientTypes;
use super::structs::recipe::{RecipeNames, Recipe};
use super::structs::ingredient::Ingredient;
use std::collections::HashMap;
use itertools::Itertools;


pub fn load_ingredient_types() -> IngredientTypes {
    // let recipe_file = std::fs::File::open(format!("./data/recipes/{}.yml", recipe)).unwrap();
    let file = std::fs::File::open("./data/ingredient_types.yml").unwrap();
    let serialized_data: IngredientTypes = serde_yaml::from_reader(file).unwrap();
    return serialized_data
}

pub fn load_measurement_types() {
    let file = std::fs::File::open("./data/measurement_types.yml").unwrap();
}

pub fn load_recipe_titles() -> RecipeNames {
    let file = std::fs::File::open("./data/recipes.yml").unwrap();
    let serialized_data: RecipeNames = serde_yaml::from_reader(file).unwrap();
    return serialized_data
}


pub fn load_recipes(recipe_names: &Vec<String>) -> Vec<Recipe> {
    let mut buffer = Vec::new();

    for name in recipe_names {
        let file_name = name.replace(" ","_");
        let recipe_file = std::fs::File::open(format!("./data/recipes/{}.yml", file_name)).unwrap();
        let serialized_data: Recipe = serde_yaml::from_reader(recipe_file).unwrap();
        buffer.push(serialized_data);
    }
    return buffer
}


pub fn process_data(size_if_bin: i32) -> Vec<(Vec<String>, f32)> {
    // items.iter().permutations(3).unique()
    let ingredients = load_ingredient_types();
    let ingredients_map = ingredients.as_map();
    let recipe_names = load_recipe_titles();
    let recipes = load_recipes(&recipe_names.names);
    let recipe_bins = recipes.iter().permutations(size_if_bin as usize);

    let mut buffer = Vec::new();

    for bin in recipe_bins {
        let recipe_combinations = process_bin(bin.clone());
        let mut bin_score = 0.0;

        for recipe_combination in recipe_combinations {
            let left_score = process_score(
                recipe_combination.0.clone(), 
                recipe_combination.1.clone().ingredient_map(), 
                ingredients_map.clone()
            );
            let right_score = process_score(
                recipe_combination.1.clone(), 
                recipe_combination.0.clone().ingredient_map(), 
                ingredients_map.clone()
            );
            // bin_buffer.push(score);
            let score = (left_score + right_score) / 2.0;
            bin_score += score;
        }
        let final_score = bin_score / size_if_bin as f32;

        let mut name_buffer = Vec::new();
        for i in bin {
            name_buffer.push(i.name.clone());
        }

        buffer.push((name_buffer, final_score));
    }
    return buffer
}


pub fn process_bin(recipe_bin: Vec<&Recipe>) -> Vec<(Recipe, Recipe)> {
    let bin_clone = recipe_bin.clone();
    let mut buffer = Vec::new();

    let end = recipe_bin.len();

    for i in 0..end {
        let left_recipe = recipe_bin[i].clone();

        for x in 0..end {
            if x != i {
                buffer.push((left_recipe.clone(), bin_clone[x].clone()));
            }
        }
    }
    return buffer
}


pub fn process_score(left_recipe: Recipe, right_recipe_ingredients: HashMap<String, i32>, ingredient_types: HashMap<String, String>) -> f32 {
    let mut total_count = 0;
    let mut hit_count = 0;
    for ingredient in left_recipe.ingredients {
        // check to see if the ingredient is "short"
        if ingredient_types.get(&ingredient.name).unwrap() == "short" {
            total_count += 1;
            // check to see if ingredient is a hit
            match right_recipe_ingredients.get(&ingredient.name) {
                Some(_) => {hit_count += 1},
                None => {}
            }
        }
    }
    let outcome = hit_count as f32 / total_count as f32;
    return outcome
}


#[cfg(test)]
mod data_tests {
    use crate::recipes::data::load_recipes;

    use super::{
        load_ingredient_types, process_score,
        load_recipe_titles, process_bin, process_data
    };
    use itertools::Itertools;


    #[test]
    fn test_load_ingredient_types() {
        let outcomes = load_ingredient_types();
        assert_eq!("salt", outcomes.accepted_types[0].name.as_str());
        assert_eq!("long", outcomes.accepted_types[0].lifetime.as_str());
    }

    #[test]
    fn test_load_recipe_titles() {
        let outcomes = load_recipe_titles();
        assert_eq!("beyond meat burger", outcomes.names[0].as_str());
    }

    #[test]
    fn test_load_recipes() {
        let names = load_recipe_titles();
        let outcomes = load_recipes(&names.names);
        assert_eq!("beyond meat burger", outcomes[0].name.as_str());
    }

    #[test]
    fn test_process_score() {
        let ingredient_types = load_ingredient_types();
        let names = load_recipe_titles();
        let recipes = load_recipes(&names.names);

        let outcome = process_score(
            recipes[1].clone(), 
            recipes[2].clone().ingredient_map(), 
            ingredient_types.as_map()
        );
        println!("here is the score: {}", outcome);
        assert_eq!(1.0, outcome);
    }

    #[test]
    fn test_process_bin() {
        // let ingredient_types = load_ingredient_types();
        let names = load_recipe_titles();
        let recipes = load_recipes(&names.names);

        // for i in process_bin(recipes) {
        //     println!("{:?}, {:?}", i.0.name, i.1.name);
        // }
        // println!("{:?}", process_bin(recipes));
    }

    #[test]
    fn test_process_data() {
        let outcome = process_data(3);

        for i in outcome {
            println!("{:?}", i);
        }
    }


}
