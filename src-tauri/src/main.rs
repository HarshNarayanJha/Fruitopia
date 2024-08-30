// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fruit::Fruit;
use reqwest::{Client, Response};

mod fruit;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn get_fruits() -> Vec<Fruit> {
    let url = String::from("https://www.fruityvice.com/api/fruit/all");

    println!("{}", url);

    let mut response = Client::new()
        .get(url)
        .send()
        .await
        .unwrap()
        .json::<Vec<Fruit>>()
        .await
        .unwrap();

    for x in response.iter_mut() {
        x.nutritions.fix();
    }

    response
}

#[tauri::command]
fn round_off(num: f32) -> f32 {
    num.round()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_fruits, round_off])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
