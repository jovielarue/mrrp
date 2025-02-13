//use diesel::prelude::*;
//use domain::models::photo::Photo;
//use domain::models::post::Post;
//use domain::schema::photos;
//use infrastructure::establish_connection;
//use rocket::{form::Form, fs::TempFile, response::status::Created};
//use shared::response_models::{Response, ResponseBody};

//pub fn upload_media<'f>(media: Form<TempFile<'f>>) -> Created<String> {
//    use domain::schema::posts;
//
//    let new_media = media.into_inner();
//    println!("{:?}", new_media);
//
//    let post: Post = Post {
//        post_id: 1,
//        description: Some("test".to_string()),
//        like_count: None,
//        song: None,
//        time: chrono::offset::Utc::now(),
//    };
//
//    let photo: Photo = Photo {
//        photo_id: 1,
//        post_id: 1,
//        description: Some("hello!".to_string()),
//        photographer: Some("Jovie".to_string()),
//        photo_path: new_media
//            .path()
//            .expect("Unable to get image path.")
//            .to_str()
//            .unwrap()
//            .to_string(),
//    };
//    //let photos: Vec<Photo> = new_post.photos;
//
//    match diesel::insert_into(posts::table)
//        .values(&post)
//        .get_result::<Post>(&mut establish_connection())
//    {
//        Ok(post) => {
//            let response = Response {
//                body: ResponseBody::Post(post),
//            };
//            println!("Db response - {:?}", response);
//            Created::new("").tagged_body(serde_json::to_string(&response).unwrap());
//        }
//        Err(err) => match err {
//            _ => {
//                panic!("Database error - {}", err);
//            }
//        },
//    }
//
//    match diesel::insert_into(photos::table)
//        .values(&photo)
//        .get_result::<Photo>(&mut establish_connection())
//    {
//        Ok(post) => {
//            let response = Response {
//                body: ResponseBody::Photo(photo),
//            };
//            println!("Db response - {:?}", response);
//            Created::new("").tagged_body(serde_json::to_string(&response).unwrap());
//        }
//        Err(err) => match err {
//            _ => {
//                panic!("Database error - {}", err);
//            }
//        },
//    }
//
//    Created::new("").tagged_body(serde_json::to_string("win").unwrap())
//}
