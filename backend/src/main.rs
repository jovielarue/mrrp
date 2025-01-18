use diesel::prelude::*;
use portable_media::establish_connection;
use portable_media::models::Post;
use rocket::tokio::time::{sleep, Duration};
use rocket::*;

#[get("/")]
fn index() -> String {
    use portable_media::schema::post::dsl::*;

    let pg_connection = &mut establish_connection();

    let results = post
        .limit(5)
        .select(Post::as_select())
        .load(pg_connection)
        .expect("Error loading posts.");

    let mut to_return = String::from("");

    for p in results {
        to_return.push_str(&p.post_id.to_string());
        to_return.push_str("\n----------------\n");
        to_return.push_str(match &p.description {
            Some(d) => &d,
            None => "No description",
        });
    }

    format!("Posts:\n\n{}", to_return)
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
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![jahsauce])
        .mount("/", routes![delay])
}
