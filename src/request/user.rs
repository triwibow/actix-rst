use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CreateUserRequest {
  pub id:u32,
  pub name:String,
  pub address:String,
  pub age:u16,
  pub gender:u8
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUserRequest {
  pub name:String,
  pub address:String,
  pub age:u16,
  pub gender:u8
}