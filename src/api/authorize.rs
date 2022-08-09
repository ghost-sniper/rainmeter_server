use rocket_contrib::json;
use rocket::Request;
use rocket::Data;
use rocket::handler::Outcome;
use rocket::http::Status;

pub fn login<'r>(req: &'r Request, data: Data<>) -> Outcome<'r> {
    if !req.content_type().map_or(false,|ct| ct.is_plain()){
        return Outcome::failure(Status::BadRequest);
    }
    Outcome::from(req, json!({"code":200,"message":"登录成功"}))
}

pub fn logout<'r>(req: &'r Request, _: Data<>) -> Outcome<'r> {
    Outcome::from(req, json!({"code":200,"message":"登出成功"}))
}