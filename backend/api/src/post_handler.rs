use application::post::read;
use domain::models::Post;
use rocket::get;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use shared::response_models::{Response, ResponseBody};

#[get("/")]
pub fn list_posts_handler() -> String {
    let posts: Vec<Post> = read::list_posts();
    let response = Response {
        body: ResponseBody::Posts(posts),
    };

    serde_json::to_string(&response).unwrap()
}

#[get("/post/<post_id>")]
pub fn list_post_handler(post_id: i32) -> Result<String, NotFound<String>> {
    let post = read::list_post(post_id)?;
    let response = Response {
        body: ResponseBody(post),
    };

    Ok(serde_json::to_string(&response).unwrap())
}
