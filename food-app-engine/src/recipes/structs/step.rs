use serde::Deserialize;


#[derive(Deserialize, Debug, Clone)]
pub struct Step {
    #[serde(rename = "TIME")]
    pub time: i32,
    #[serde(rename = "TIME_TYPE")]
    pub time_type: String,
    #[serde(rename = "INGREDIENTS")]
    pub ingredients: Vec<String> 
}
