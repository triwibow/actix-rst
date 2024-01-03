mod user;

use actix_web::{web, Scope};

pub fn all_handler() -> Scope {
  web::scope("")
    .service(user::get_handler())
}

