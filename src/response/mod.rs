use serde::Serialize;
use actix_web::web;

#[derive(Serialize)]
pub struct Res<T> {
  status:String,
  data:T
}

impl<T> Res<T>{
  pub fn json(status:String, data:T) -> web::Json<Self> {
    web::Json(
      Self {
        status,
        data
      }
    )
  }
}