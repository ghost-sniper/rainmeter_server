use rocket::{Rocket, Route};
use rocket::http::Method::{Get, Post};
use crate::api::authorize::login;
use crate::api::authorize::logout;

mod authorize;

pub fn register_api() -> Rocket {
    let login = Route::new(Post, "/login", login);
    let logout = Route::new(Get, "/logout", logout);
    rocket::ignite().mount("/", vec![login, logout])
}