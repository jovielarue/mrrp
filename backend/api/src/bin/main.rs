use api::{auth_handler, post_handler};
use rocket::*;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(CORS)
        .mount(
            "/api",
            routes![
                post_handler::list_posts_handler,
                post_handler::list_post_handler,
                post_handler::create_post_handler,
                post_handler::delete_post_handler,
                post_handler::edit_post_handler,
            ],
        )
        .mount(
            "/auth",
            routes![auth_handler::authenticate, auth_handler::sign_up],
        )
}

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
