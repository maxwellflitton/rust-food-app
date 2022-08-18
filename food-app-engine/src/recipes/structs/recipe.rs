use serde::Deserialize;
use super::ingredient::Ingredient;
use super::step::Step;
use std::collections::HashMap;


#[derive(Deserialize, Debug)]
pub struct RecipeNames {
    #[serde(rename = "RECIPES")]
    pub names: Vec<String>
}


#[derive(Deserialize, Debug, Clone)]
pub struct Recipe {
    #[serde(rename = "NAME")]
    pub name: String,
    #[serde(rename = "TYPE")]
    pub recipe_type: String,
    #[serde(rename = "COUNTRY")]
    pub country: String,
    #[serde(rename = "DINNER_TYPE")]
    pub dinner_type: Vec<String>,
    #[serde(rename = "INGREDIENTS")]
    pub ingredients: Vec<Ingredient>,
    #[serde(rename = "STEPS")]
    pub steps: Vec<Step>,
}

impl Recipe {

    pub fn ingredient_map(self) -> HashMap<String, i32> {
        let mut map = HashMap::new();
        for ingredient in self.ingredients {
            map.insert(ingredient.name.clone(), 0);
        }
        return map
    }

}
