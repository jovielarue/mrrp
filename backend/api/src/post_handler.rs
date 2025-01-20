use application::post::{create, delete, read};
use domain::models::Post;
use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket::{get, post};
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
        body: ResponseBody::Post(post),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/new_post", format = "application/json", data = "<post>")]
pub fn create_post_handler(post: Json<Post>) -> Created<String> {
    create::create_post(post)
}

#[get("/delete/<post_id>")]
pub fn delete_post_handler(post_id: i32) -> Result<String, NotFound<String>> {
    let posts = delete::delete_post(post_id)?;
    let response = Response {
        body: ResponseBody::Posts(posts),
    };

    Ok(serde_json::to_string(&response).unwrap())
}
