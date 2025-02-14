#[macro_use] extern crate rocket;

use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::State;
use std::sync::Mutex;
use serde_json;

#[derive(Serialize, Deserialize, Clone)]
struct Item {
    id: u32,
    name: String,
    quantity: u32,
}

type Inventory = Mutex<Vec<Item>>;

#[get("/items")]
fn get_items(inventory: &State<Inventory>) -> String {
    let items = inventory.lock().unwrap();
    serde_json::to_string(&*items).unwrap()
}

#[post("/items", format = "json", data = "<new_item>")]
fn create_item(inventory: &State<Inventory>, new_item: Json<Item>) -> String {
    let mut items = inventory.lock().unwrap();
    for item in items.iter(){
        if item.id==new_item.id || item.name==new_item.name {
            let ret_string = serde_json::to_string(&*items).unwrap();
            let warning = String::from("Item with the same ID (or) Name already present");
            return format!("{}\n{}", ret_string, warning);
        }
    }
    items.push(new_item.into_inner());
    return serde_json::to_string(&*items).unwrap();
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Mutex::new(Vec::<Item>::new()))  // in-memory storage
        .mount("/", routes![get_items, create_item])
}