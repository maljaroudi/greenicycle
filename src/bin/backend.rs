#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use mytodo::db::{query_task, establish_connection};

#[get("/items")]
fn tasks_get() -> String {
    let mut response: Vec::<String> = vec![];
    let conn = establish_connection();
    //response.push("Product  Category    Recyclable\n");
    for task in query_task(&conn) {
        response.push([task._name, "    ".to_string(), task.category, "     ".to_string(), task.recyclable].into_iter().map(|i| i.to_string()).collect::<String>());
    }
    response.join("\n")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![tasks_get])
        .launch();
}