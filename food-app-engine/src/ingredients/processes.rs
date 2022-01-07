use serde_yaml;
use std::collections::BTreeMap;

use super::ingredients_map::IngredientsShopMap;


pub fn load_meta_data(file: &str) -> Vec<String> {
    let file = std::fs::File::open(format!("./data/{}.yml", file)).unwrap();
    let map: BTreeMap<String, serde_yaml::Value> = serde_yaml::from_reader(file).unwrap();
    return map.get("ACCEPTED_TYPES").unwrap().as_sequence().unwrap().iter().map(|x| String::from(x.as_str().unwrap())).collect();
}


pub fn load_recipe_list(recipe: String, mut ingredients_map: IngredientsShopMap) -> IngredientsShopMap {
    let recipe_file = std::fs::File::open(format!("./data/recipes/{}.yml", recipe)).unwrap();
    let recipe_map: BTreeMap<String, serde_yaml::Value> = serde_yaml::from_reader(recipe_file).unwrap();

    let ingredients: &Vec<serde_yaml::Value> = recipe_map.get("INGREDIENTS").unwrap().as_sequence().unwrap();

    for ingredient in ingredients {

        let map = ingredient.as_mapping().unwrap();
        let name = String::from(map.get(&serde_yaml::Value::from("NAME")).unwrap().as_str().unwrap());
        let amount = map.get(&serde_yaml::Value::from("AMOUNT")).unwrap().as_f64().unwrap() as f32;
        let measurement = String::from(map.get(&serde_yaml::Value::from("MEASUREMENT_TYPE")).unwrap().as_str().unwrap());

        ingredients_map = ingredients_map.to_owned().insert_ingredient(name, amount, measurement);
    }

    // TODO => this is mapping other recipies attached to the recipe
    // match recipe_map.get("OTHER_RECIPES") {
    //     Some(data) => {
    //         let other_recipes = data.as_sequence().unwrap();
    //         for recipe in other_recipes {
    //             let name = recipe.as_mapping().unwrap().get(&serde_yaml::Value::from("NAME")).unwrap().as_str().unwrap();
    //             ingredients_map = load_recipe_list(String::from(name), ingredients_map);
    //         }
    //     }
    //     None => {}
    // }

    return ingredients_map
} 


pub fn get_recipes() -> Vec<String> {
    let file = std::fs::File::open("./data/recipes.yml").unwrap();
    let recipe_map: BTreeMap<String, serde_yaml::Value> = serde_yaml::from_reader(file).unwrap();
    let recipes = recipe_map.get("RECIPES").unwrap().as_sequence().unwrap().iter().map(|x| String::from(x.as_str().unwrap())).collect();
    return recipes
}