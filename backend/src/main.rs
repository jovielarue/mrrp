use diesel::prelude::*;
use portable_media::establish_connection;
use portable_media::models::Post;
use rocket::tokio::time::{sleep, Duration};
use rocket::*;

#[get("/")]
fn index() -> &'static str {
    println!("hello");
    "Hello, World!"
}

#[get("/jahsauce")]
fn jahsauce() -> &'static str {
    "Yippee!"
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[launch]
fn rocket() -> Rocket<Build> {
    use portable_media::schema::post::dsl::*;

    let pg_connection = &mut establish_connection();

    let results = post
        .limit(5)
        .select(Post::as_select())
        .load(pg_connection)
        .expect("Error loading posts.");

    for p in results {
        println!("{}", p.post_id);
        println!("-----------");
        println!(
            "{}",
            match p.description {
                Some(d) => d,
                None => "No description".to_string(),
            }
        );
    }

    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![jahsauce])
        .mount("/", routes![delay])
}
