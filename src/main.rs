use std::path::Path;
use std::fs::File;

use serde::{Serialize, Deserialize};
use todo_list::Task;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // let point = Point { x: 1, y: 2 };

    // let serialized = serde_json::to_string(&point).unwrap();
    // println!("serialized = {}", serialized);

    // let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    // println!("deserialized = {:?}", deserialized);

    let json_file_path = Path::new("database.json");
    let file = File::open(json_file_path).expect("file not found");


    let tasks:Vec<Task> = serde_json::from_reader(file).expect("error while reading");
    for task in tasks.iter() {
        println!("{}", task);
    }

}