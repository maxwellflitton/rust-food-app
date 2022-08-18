use serde::Deserialize;
use std::collections::HashMap;


#[derive(Deserialize, Debug)]
pub struct IngredientType {
    #[serde(rename = "NAME")]
    pub name: String,
    #[serde(rename = "LIFETIME")]
    pub lifetime: String
}


#[derive(Deserialize, Debug)]
pub struct IngredientTypes {
    #[serde(rename = "ACCEPTED_TYPES")]
    pub accepted_types: Vec<IngredientType>
}


impl IngredientTypes {

    pub fn as_map(self) -> HashMap<String, String> {
        let mut map = HashMap::new();

        for ingredient in self.accepted_types {
            map.insert(ingredient.name.clone(), ingredient.lifetime.clone());
        }
        return map
    }

}
