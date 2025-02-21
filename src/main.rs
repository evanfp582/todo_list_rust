mod task;
mod task_backend;

use task_backend::crud::{refresh_json, read_json_db};

use crate::task::Task;

fn main() {

    let mut tasks:Vec<Task> = read_json_db("database.json");
    println!("{}", serde_json::to_string(&tasks).expect("JSON to String failed"));

    let new_task = Task::new("Meowed".to_owned(), false);

    tasks.push(new_task);

    println!("{}", serde_json::to_string(&tasks).expect("JSON to String failed"));

    let _ = refresh_json("database.json", &tasks);

    // for task in tasks.iter() {
    //     println!("{}", task);
    //     let _ = add_task_to_json("database.json", task);
    // }

}