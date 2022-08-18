use serde::Deserialize;


#[derive(Deserialize, Debug, Clone)]
pub struct Ingredient {
    #[serde(rename = "NAME")]
    pub name: String,
    #[serde(rename = "AMOUNT")]
    pub amount: f32,
    #[serde(rename = "MEASUREMENT_TYPE")]
    pub measurement_type: String
}
