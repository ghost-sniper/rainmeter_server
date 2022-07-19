use rocket_contrib::json;
use rocket::Request;
use rocket::Data;
use rocket::handler::Outcome;

pub fn login<'r>(req: &'r Request, _: Data<>) -> Outcome<'r> {
    Outcome::from(req, json!({"code":200,"message":"登录成功"}))
}

pub fn logout<'r>(req: &'r Request, _: Data<>) -> Outcome<'r> {
    Outcome::from(req, json!({"code":200,"message":"登出成功"}))
}