use std::collections::HashMap;


//// This struct is for keeping track of amount needed for an ingredient.
/// 
/// # Attributes
/// - amount (f32): the amount of ingredient needed
/// - measurement (String): the type measurement needed for the ingredient
pub struct IngredientMeasurement {
    pub amount: f32,
    pub measurement: String
}


pub struct IngredientsShopMap {
    pub amount_map: HashMap<String, IngredientMeasurement>,
    pub allowed_measurements: Vec<String>,
    pub allowed_ingredients: Vec<String>
}

impl IngredientsShopMap {

    pub fn new(measurements: Vec<String>, ingredients: Vec<String>) -> IngredientsShopMap {
        let new_map: HashMap<String, IngredientMeasurement> = HashMap::new();
        return IngredientsShopMap{
            amount_map: new_map,
            allowed_measurements: measurements,
            allowed_ingredients: ingredients
        }
    }

    fn insert_ingredient(mut self, name: String, amount: f32, measurement: String) -> Self {

        if self.allowed_ingredients.contains(&name) && self.allowed_measurements.contains(&measurement) {
            
            match self.amount_map.get(&name) {
                Some(data) => {
                    let new_amount = data.amount + amount;
                    let new_measurement = IngredientMeasurement{amount: new_amount, measurement: measurement};
                    self.amount_map.insert(name, new_measurement);
                    return self
                }
                None => {
                    let new_measurement = IngredientMeasurement{amount, measurement};
                    self.amount_map.insert(name, new_measurement);
                    return self
                }
            }

        } else {
            panic!("ingredient {} or measurement {} was not allowed", name, measurement);
        }
    }

}
