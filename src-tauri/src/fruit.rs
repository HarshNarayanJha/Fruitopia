use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Nutritions {
    pub calories: f32,
    pub fat: f32,
    pub sugar: f32,
    pub carbohydrates: f32,
    pub protein: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fruit {
    pub name: String,
    pub id: i32,
    pub family: String,
    pub order: String,
    pub genus: String,
    pub nutritions: Nutritions,
}
