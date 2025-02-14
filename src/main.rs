#[macro_use] extern crate rocket;

use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::{Data, Request, State};
use rocket::http::{Cookie, CookieJar};
use std::sync::Mutex;
use std::collections::HashMap;
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

#[post("/signup", format="json", data="<user>")]
fn signup(user: Json<User>, user_store: &State<UserStore>)->String{
    let mut users = user_store.lock().unwrap();
    if users.iter().any(|u| u.username==user.username) {
        return "Username already exists".to_string();
    }
    users.push(user.into_inner());
    "User created successfully!".to_string()
}

#[post("/login", format="json", data="<user>")]
fn login(user_store:&State<UserStore>, user: Json<User>, cookies: &CookieJar)->String{
    let users = user_store.lock().unwrap();
    if let Some(u) = users.iter().find(|u| u.username == user.username && u.password==user.password){
        cookies.add(Cookie::new("username", u.username.clone()));
        "Login successful".to_string()
    } else {
        "Invalid Username or password".to_string()
    }
}

#[get("/logout")]
fn logout(cookies: &CookieJar){
    cookies.remove(Cookie::from("username"));
    //Redirect::to("/")
}

#[get("/me")]
fn me(cookies: &CookieJar) -> String{
    if let Some(cookie) = cookies.get("username"){
        format!("Currently logged in as: {}", cookie.value().to_string())
    } else {
        "Not logged in".to_string()
    }
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

#[get("/items/<id>")]
fn get_items_by_id(cookies: &CookieJar, id: u32, inventory: &State<Inventory>)->Result<String, String>{
    if let Some(cookie) = cookies.get("username"){
        let username = cookie.value().to_string();
        if let Some(items) = inventory.lock().unwrap().get(&username){
            if let Some(item) = items.iter().find(|i| i.id==id){
                Ok(serde_json::to_string(item).unwrap())
            } else {
                Err(format!("Item with id:{id} not found"))
            }
        } else {
            Err("No items found for the user".to_string())
        }
    } else {
        Err("You must be logged in to view your inventory".to_string())
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

#[put("/items/update/<id>", format="json", data="<updated_item>")]
fn update_by_id(inventory: &State<Inventory>, updated_item: Json<Item>, cookies: &CookieJar, id:u32)->Result<String, String>{
    if let Some(cookie) = cookies.get("username"){
        let username = cookie.value().to_string();
        if let Some(items) = inventory.lock().unwrap().get_mut(&username){
            if let Some(item) = items.iter_mut().find(|i| i.id==id) {
                item.name = updated_item.name.clone();
                item.quantity = updated_item.quantity;

                return Ok(serde_json::to_string(item).unwrap())
            } else {
                return Err(format!("Item with id:{id} not present"));
            }
        } else {
            return Err("No items for the present user".to_string());
        }
    } else {
        return Err("You must be logged in to updated item.".to_string());
    }
}

#[put("/items/delete/<id>")]
fn delete_by_id(inventory: &State<Inventory>, id: u32, cookies: &CookieJar)->Result<String, String>{
    if let Some(cookie) = cookies.get("username"){
        let username = cookie.value().to_string();
        if let Some(items) = inventory.lock().unwrap().get_mut(&username){
            items.retain(|i| i.id!=id);
            return Ok(format!("Item with id:{id}, successfully deleted."));
        } else {
            return Err(format!("Could not retrieve the item with id:{id}"));
        }
    } else {
        return Err("You must be logged in!".to_string());
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Mutex::new(HashMap::<String, Vec::<Item>>::new()))  // in-memory storage
        .manage(Mutex::new(Vec::<User>::new()))  // in-memory storage
        .mount("/", routes![get_items, get_items_by_id, create_item, update_by_id, delete_by_id, signup, login, logout, me])
}