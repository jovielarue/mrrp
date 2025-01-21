use diesel::prelude::*;
use domain::models::{Photo, Post};
use infrastructure::establish_connection;
use rocket::{form::Form, fs::TempFile, response::status::Created};
use shared::response_models::{Response, ResponseBody};

pub fn upload_media<'f>(media: Form<TempFile<'f>>) -> Created<String> {
    use domain::schema::posts;

    let new_post = media.into_inner();
    println!("{:?}", new_post);

    //let post: Post = new_post.post;
    ////let photos: Vec<Photo> = new_post.photos;
    //
    //match diesel::insert_into(posts::table)
    //    .values(&post)
    //    .get_result::<Post>(&mut establish_connection())
    //{
    //    Ok(post) => {
    //        let response = Response {
    //            body: ResponseBody::Post(post),
    //        };
    //        println!("Db response - {:?}", response);
    //        Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
    //    }
    //    Err(err) => match err {
    //        _ => {
    //            panic!("Database error - {}", err);
    //        }
    //    },
    //}
    Created::new("").tagged_body(serde_json::to_string("win").unwrap())
}
