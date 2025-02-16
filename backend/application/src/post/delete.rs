use diesel::prelude::*;
use domain::models::post::Post;
use infrastructure::establish_connection;
use rocket::response::status::NotFound;
use shared::response_models::{Response, ResponseBody};

pub fn delete_post(id: i32) -> Result<Vec<Post>, NotFound<String>> {
    use domain::schema::posts::dsl::*;

    let response: Response;

    let num_deleted =
        match diesel::delete(posts.filter(post_id.eq(id))).execute(&mut establish_connection()) {
            Ok(count) => count,
            Err(err) => match err {
                diesel::result::Error::NotFound => {
                    let response = Response {
                        body: ResponseBody::Message(format!(
                            "Error deleting post with id {:?} - {}",
                            post_id, err
                        )),
                    };
                    Err(NotFound(serde_json::to_string(&response).unwrap()))?
                }
                _ => {
                    return Err(NotFound(format!("Database error: {}", err)));
                }
            },
        };

    if num_deleted > 0 {
        match posts
            .select(posts::all_columns())
            .load::<Post>(&mut establish_connection())
        {
            Ok(mut posts_) => {
                posts_.sort();
                return Ok(posts_);
            }
            Err(err) => match err {
                _ => {
                    return Err(NotFound(format!("Database error: {}", err)));
                }
            },
        }
    } else {
        response = Response {
            body: ResponseBody::Message(format!("Error - no post with id {}", id)),
        };
        Err(NotFound(serde_json::to_string(&response).unwrap()))?
    }
}
