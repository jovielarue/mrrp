use diesel::prelude::*;
use domain::models::post::PostReturn;
use infrastructure::establish_connection;
use rocket::response::status::NotFound;

pub fn edit_post(id: i32, new_post_text: &str) -> Result<PostReturn, NotFound<String>> {
    use domain::schema::posts::dsl::*;

    let _ = diesel::update(posts)
        .set(text.eq(new_post_text))
        .execute(&mut establish_connection());

    let new_post = crate::post::read::list_post(id)?;

    Ok(new_post)
}
