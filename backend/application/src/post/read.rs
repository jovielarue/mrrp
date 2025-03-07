use diesel::prelude::*;
use domain::models::{
    post::{Post, PostReturn},
    user::User,
};
use infrastructure::establish_connection;
use rocket::response::status::NotFound;

pub fn list_post(post_id: i32) -> Result<PostReturn, NotFound<String>> {
    use domain::schema::posts;

    let post = match posts::table
        .find(post_id)
        .first::<Post>(&mut establish_connection())
    {
        Ok(post) => Ok(post),
        Err(e) => {
            eprintln!("{e}");
            Err(NotFound("Post not found.".to_string()))
        }
    }?;

    let found_user = find_user_by_post(&post);
    Ok(PostReturn {
        post,
        username: found_user.username,
    })
}

pub fn list_posts() -> Result<Vec<PostReturn>, diesel::result::Error> {
    use domain::schema::posts;

    let posts = match posts::table
        .select(posts::all_columns)
        .load::<Post>(&mut establish_connection())
    {
        Ok(post_list) => Ok(post_list),
        Err(err) => Err(err),
    }?;

    let post_list: Vec<PostReturn> = posts
        .into_iter()
        .rev()
        .map(|post| PostReturn {
            post: post.clone(),
            username: find_user_by_post(&post).username,
        })
        .collect();

    Ok(post_list)
}

pub fn find_user_by_post(post: &Post) -> User {
    use domain::schema::users;
    use domain::schema::users::dsl::*;

    let found_user = match users::table
        .filter(user_id.eq(post.user_id))
        .select(User::as_select())
        .get_result::<User>(&mut establish_connection())
    {
        Ok(user) => Some(user),
        _ => None,
    }
    .unwrap();

    found_user
}
