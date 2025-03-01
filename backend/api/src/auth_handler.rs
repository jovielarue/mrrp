use domain::models::user::UserForm;
use rocket::{form::Form, post};
use sha2::{Digest, Sha256};

#[post("/login", format = "multipart/form-data", data = "<user>")]
pub fn authenticate(
    user: Form<UserForm>,
) -> Result<String, rocket::response::status::Unauthorized<String>> {
    let mut hasher = Sha256::new();
    hasher.update(&user.password);
    let hashed: &str = &format!("{:X}", hasher.finalize());
    println!("{hashed}");
    let user = application::user::find::find_user_id(&user.username);

    if let Some(user) = user {
        println!("{}", user.password);
        if hashed == &user.password {
            return Ok(serde_json::to_string("jwt").unwrap());
        }
    }

    Err(rocket::response::status::Unauthorized::<String>(
        "You are not authorized.".to_string(),
    ))
}

#[post("/signup", format = "multipart/form-data", data = "<user>")]
pub fn sign_up(
    user: Form<UserForm>,
) -> Result<String, rocket::response::status::Unauthorized<String>> {
    let username = user.username.clone();
    let password = user.password.clone();

    let mut hasher = Sha256::new();
    hasher.update(password);
    let hashed = format!("{:X}", hasher.finalize());

    let user_id = application::user::create::create_user(&username, hashed);

    if user_id != -1 {
        return Ok(serde_json::to_string("jwt").unwrap());
    } else {
        Err(rocket::response::status::Unauthorized::<String>(
            serde_json::to_string("You are not authorized.").unwrap(),
        ))
    }
}
