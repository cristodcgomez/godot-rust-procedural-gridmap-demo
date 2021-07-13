use gdnative::prelude::*;

#[derive(Debug)]
pub struct ItemBag {
  pub id: i32,
  pub name: String,
  pub amount: i32
}

pub struct Bag {
  pub objects: Vec<ItemBag>,
  pub tools: Int32Array,
  pub keys: Int32Array,
  pub number_items: i32,
  pub total_items: i32,
}
impl Bag {
  pub fn new() -> Self {
    Bag {
      objects: Vec::new(),
      tools: Int32Array::new(),
      keys: Int32Array::new(),
      number_items: 0,
      total_items: 10,
    }
  }
}
