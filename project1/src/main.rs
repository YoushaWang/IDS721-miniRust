// fn main() {
//     println!("Hello, world!");
// }
#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{get, post, routes, response::status::Accepted};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    title: String,
    completed: bool
}

#[get("/")]
fn index() -> &'static str {
    "Welcome to the To-Do List Microservice"
}

#[get("/tasks")]
fn get_tasks() -> rocket_contrib::json::Json<Vec<Task>> {
    rocket_contrib::json::Json(TASKS.read().unwrap().values().cloned().collect())
}

#[post("/tasks", format = "json", data = "<task>")]
fn create_task(task: rocket_contrib::json::Json<Task>) -> Accepted<String> {
    let mut tasks = TASKS.write().unwrap();
    let id = tasks.keys().last().map(|&id| id + 1).unwrap_or(0);
    tasks.insert(id, task.into_inner());
    Accepted("Task created".into())
}

#[get("/tasks/<id>")]
fn get_task(id: u32) -> Option<rocket_contrib::json::Json<Task>> {
    TASKS.read().unwrap().get(&id).map(rocket_contrib::json::Json)
}

#[put("/tasks/<id>", format = "json", data = "<task>")]
fn update_task(id: u32, task: rocket_contrib::json::Json<Task>) -> Option<rocket_contrib::json::Json<Task>> {
    let mut tasks = TASKS.write().unwrap();
    tasks.insert(id, task.into_inner()).map(rocket_contrib::json::Json)
}

#[delete("/tasks/<id>")]
fn delete_task(id: u32) -> Option<String> {
    TASKS.write().unwrap().remove(&id).map(|_| "Task deleted".into())
}

lazy_static! {
    static ref TASKS: rocket::sync::RwLock<HashMap<u32, Task>> = rocket::sync::RwLock::new(HashMap::new());
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, get_tasks, create_task, get_task, update_task, delete_task])
        .launch();
}
