use actix_web::{get, post, web, Scope, Result, Responder};
use crate::response::Res;
use crate::model::user::UserModel;
use crate::request::user::{CreateUserRequest, UpdateUserRequest};


pub fn get_handler()-> Scope {
  web::scope("/user")
    .service(index)
    .service(user_by_id)
    .service(create)
    .service(update_by_id)
}

#[get("")]
async fn index() -> Result<impl Responder> {

  let data = vec![
    UserModel::new(12, String::from("Sgie"), String::from("Sarkem"), 12, 1),
    UserModel::new(12, String::from("Sgie"), String::from("Sarkem"), 12, 1)
  ];

  Ok(Res::json(
    String::from("success"),
    data
  ))
}

#[post("")]
async fn create(request:web::Json<CreateUserRequest>) -> Result<impl Responder> {

  let payload = request;

  Ok(Res::json(
    String::from("success"),
    payload
  ))

}

#[get("/{id}")]
async fn user_by_id(path: web::Path<(u32,)>) -> Result<impl Responder> {
  let id = path.into_inner().0;
  let data = UserModel::new(
    id,
    String::from("Joni"),
    String::from("Sedayu"),
    12,
    2
  );

  Ok(Res::json(
    String::from("success"),
    data
  ))
}

#[post("/{id}")]
async fn update_by_id(path:web::Path<(u32,)>, request:web::Json<UpdateUserRequest>) -> Result<impl Responder> {
  let id = path.into_inner().0;

  let data = UserModel::new(
    id,
    request.name.clone(),
    request.address.clone(),
    request.age,
    request.gender
  );

  Ok(Res::json(
    String::from("success"),
    data
  ))  
}

