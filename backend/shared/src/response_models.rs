use domain::models::{
    post::{Post, PostReturn},
    user::User,
};
use rocket::serde::Serialize;

#[derive(Serialize, Debug)]
pub enum ResponseBody {
    Message(String),
    Post(PostReturn),
    Posts(Vec<Post>),
    PostReturns(Vec<PostReturn>),
    User(User),
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}
