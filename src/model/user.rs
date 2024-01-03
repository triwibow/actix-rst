use serde::Serialize;

#[derive(Serialize)]
pub struct UserModel {
  id:u32,
  name:String,
  address:String,
  age:u16,
  gender:u8
}

impl UserModel {
  pub fn new(id:u32, name:String, address:String, age:u16, gender:u8) -> Self {
    Self {
      id,
      name,
      address,
      age,
      gender
    }
  }
}