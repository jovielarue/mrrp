use rocket::tokio::time::{sleep, Duration};
use rocket::*;

#[get("/")]
fn index() -> &'static str {
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
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![jahsauce])
        .mount("/", routes![delay])
}
