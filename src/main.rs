#[macro_use] extern crate rocket;

use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::State;
use rocket::response::Redirect;
use rocket::http::{Cookie, CookieJar};
use std::sync::Mutex;
use std::collections::HashMap;
use std::path::Component::ParentDir;
use serde_json;

#[derive(Serialize, Deserialize, Clone)]
struct Item {
    id: u32,
    name: String,
    quantity: u32,
}

#[derive(Serialize, Deserialize, Clone)]
struct User {
    username: String,
    password: String,
}

type Inventory = Mutex<HashMap<String, Vec<Item>>>;
type UserStore = Mutex<Vec<User>>;

#[post("/signup"), format="json", data="<user>"]
fn signup(user: Json<User>, user_store: &State<UserStore>)->String{
    let mut users = user_store.lock().unwrap();
    if users.iter().any(|u| u.username==user.username) {
        return "Username already exists".to_string();
    }
    let nu = &user.username;
    users.push(user.into_inner());
    format!("User {}, created successfully!", nu)
}

#[post("/login"), format="json", data="<user>"]
fn login(user_store:&State<UserStore>, user: Json<User>, cookies: &CookieJar)->String{
    let users = user_store.lock().unwrap();
    if let Some(u) = users.iter().find(|u| u.username == user.username && u.password==user.password){
        cookies.add(Cookie::new("username", u.username.clone()));
        "Login successful".to_string()
    } else {
        "Invalid Username or password".to_string()
    }
}

#[post("/logout")]
fn logout(cookies: &CookieJar) -> Redirect {
    cookies.remove(Cookie::from("username"));
    Redirect::to("/")
}

#[get("/items")]
fn get_items(inventory: &State<Inventory>, cookies: &CookieJar) -> Result<String, String> {
    if let Some(cookie) = cookies.get("username"){
        let username = cookie.value().to_string();
        let inventory = inventory.lock().unwrap();
        if let Some(items) = inventory.get(&username){
            Ok(serde_json::to_string(items).unwrap())
        } else {
            Ok("[]".to_string())
        }
    } else {
        Err("You must be logged in".to_string())
    }
}

#[post("/items", format = "json", data = "<new_item>")]
fn create_item(inventory: &State<Inventory>, new_item: Json<Item>, cookies: &CookieJar) -> Result<String, String> {
    if let Some(cookie) = cookies.get("username"){
        let username = cookie.value().to_string();
        let mut inventory = inventory.lock().unwrap();
        let items = inventory.entry(username).or_insert_with(Vec::new);
        if items.iter().any(|i| i.id==new_item.id || i.name==new_item.name){
            let ret_string = serde_json::to_string(&*items).unwrap();
            let warning = String::from("Item with the same ID (or) Name already present");
            return Ok(format!("{}\n{}", ret_string, warning));
        }
        items.push(new_item.into_inner());
        Ok(serde_json::to_string(&*items).unwrap())
    } else {
        Err("You must be logged in to add items".to_string())
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Mutex::new(Vec::<Item>::new()))  // in-memory storage
        .mount("/", routes![get_items, create_item])
}