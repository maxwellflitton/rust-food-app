#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use ingredients::processes::get_recipes;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};

mod ingredients;
use ingredients::ingredients_map::IngredientsShopMap;
use ingredients::processes::{load_meta_data, load_recipe_list};


#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}


#[derive(Serialize, Deserialize)]
pub struct RecipesResponse {
    pub recipes: Vec<String>
}


#[get("/get/all")]
fn get_all_recipes() -> Json<RecipesResponse> {
    let mut test = Vec::new();
    test.push(String::from("one"));
    let outcome = RecipesResponse{recipes: get_recipes()};
    return Json(outcome)
}

#[post("/create/shopping", data="<recipes>", format = "json")]
fn create_shopping(recipes: Json<RecipesResponse>) -> Json<RecipesResponse> {
    let measurements = load_meta_data("measurement_types");
    let ingredients = load_meta_data("ingredient_types");
    let mut map = IngredientsShopMap::new(measurements, ingredients);

    for i in recipes.into_inner().recipes {
        map = load_recipe_list(i, map);
    }

    let mut buffer = Vec::new();

    for (key, value) in &map.amount_map {
        let amount = format!("{} : {} {}", key, value.amount, value.measurement);
        buffer.push(amount);
    }

    let outcome = RecipesResponse{recipes: buffer};
    return Json(outcome)
}



fn main() {
    let cors = CorsOptions::default()
    .allowed_origins(AllowedOrigins::all())
    .allowed_methods(
        vec![Method::Get, Method::Post, Method::Patch]
            .into_iter()
            .map(From::from)
            .collect(),
    )
    .allow_credentials(true);
    rocket::ignite().mount("/", routes![hello, get_all_recipes, create_shopping]).attach(cors.to_cors().unwrap()).launch();
}

