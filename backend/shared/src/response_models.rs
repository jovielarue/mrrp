use domain::models::{photo::Photo, post::Post};
use rocket::serde::Serialize;

#[derive(Serialize, Debug)]
pub enum ResponseBody {
    Message(String),
    Post(Post),
    Posts(Vec<Post>),
    Photo(Photo),
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}
