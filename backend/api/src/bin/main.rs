use api::post_handler;
use rocket::*;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount(
        "/api",
        routes![
            post_handler::list_posts_handler,
            post_handler::list_post_handler,
            post_handler::create_post_handler,
        ],
    )
}
