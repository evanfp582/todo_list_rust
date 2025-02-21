use std::path::Path;
use std::fs::File;
use std::fs;
use serde_json::Result;

use crate::task::Task;


pub fn refresh_json(path_str:&str, tasks: &Vec<Task>) -> Result<()>{
  let json = serde_json::to_string(tasks)?;
  fs::write(path_str, json).expect("Write Failed");

  Ok(())
}

pub fn read_json_db(path_str:&str) -> Vec<Task> {
  let json_file_path = Path::new(path_str);
  let file = File::open(json_file_path).expect("file not found");

  let tasks:Vec<Task> = serde_json::from_reader(file).expect("error while reading");
  return  tasks;
}