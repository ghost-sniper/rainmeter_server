extern crate core;

use crate::api::register_api;

mod api;
mod model;
mod common;


fn main() {
    register_api().launch();
}