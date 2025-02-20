use core::fmt;
use std::str;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
  id: u8,
  title: String,
  checked: bool,
  position: u8
}

impl Task {
  pub fn new(title: String, checked: bool) -> Result<()>{
    println!("Created Task with title {}, checked {}, position LAST", title, checked);
    Ok(())
  }
}

impl fmt::Display for Task {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      // Customize so only `x` and `y` are denoted.
      write!(f, "title: {}, checked: {}", self.title, self.checked)
  }
}
