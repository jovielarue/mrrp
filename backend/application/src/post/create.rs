use diesel::prelude::*;
use domain::models::post::{Post, PostForm};
use infrastructure::establish_connection;
use rocket::{form::Form, response::status::Created};
use shared::response_models::{Response, ResponseBody};

pub fn create_post(post_form: Form<PostForm>) -> Created<String> {
    use domain::schema::posts;

    let post_form = post_form.into_inner();
    let post: Post = Post {
        post_id: 0,
        username: post_form.username,
        text: post_form.post,
        like_count: None,
        time: chrono::offset::Utc::now(),
    };
    println!("{:?}", post);

    match diesel::insert_into(posts::table)
        .values(&post)
        .get_result::<Post>(&mut establish_connection())
    {
        Ok(post) => {
            let response = Response {
                body: ResponseBody::Post(post),
            };
            println!("Db response - {:?}", response);
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        }
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
