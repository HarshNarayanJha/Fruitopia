use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Nutritions {
    pub calories: f32,
    pub fat: f32,
    pub sugar: f32,
    pub carbohydrates: f32,
    pub protein: f32,
}

impl Nutritions {
    pub fn fix(&mut self) {
        self.calories = (self.calories * 100.0).round() / 100.0;
        self.fat = (self.fat * 100.0).round() / 100.0;
        self.sugar = (self.sugar * 100.0).round() / 100.0;
        self.carbohydrates = (self.carbohydrates * 100.0).round() / 100.0;
        self.protein = (self.protein * 100.0).round() / 100.0;
    }
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
