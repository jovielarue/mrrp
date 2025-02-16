use diesel::prelude::*;
use domain::models::post::Post;
use infrastructure::establish_connection;
use rocket::response::status::NotFound;
use shared::response_models::{Response, ResponseBody};

pub fn list_post(post_id: i32) -> Result<Post, NotFound<String>> {
    use domain::schema::posts;

    match posts::table
        .find(post_id)
        .first::<Post>(&mut establish_connection())
    {
        Ok(post) => Ok(post),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response {
                    body: ResponseBody::Message(format!(
                        "Error selecting post with id {} - {}",
                        post_id, err
                    )),
                };
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            }
            _ => {
                return Err(NotFound(format!("Database error: {}", err)));
            }
        },
    }
}

pub fn list_posts() -> Result<Vec<Post>, diesel::result::Error> {
    use domain::schema::posts;

    match posts::table
        .select(posts::all_columns)
        .load::<Post>(&mut establish_connection())
    {
        Ok(mut post_list) => {
            post_list.sort();
            Ok(post_list)
        }
        Err(err) => Err(err),
    }
}
