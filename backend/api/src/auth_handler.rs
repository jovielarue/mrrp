use hmac::{Hmac, Mac};
use std::{collections::BTreeMap, env, io::Error};

use domain::models::user::{User, UserForm};
use dotenvy::dotenv;
use jwt::{SignWithKey, VerifyWithKey};
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
            return Ok(serde_json::to_string(
                &generate_jwt(&user.username).expect("Unable to generate jwt."),
            )
            .unwrap());
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

    if user_id != Ok(-1) {
        return Ok(serde_json::to_string(
            &generate_jwt(&username).expect("Unable to generate jwt."),
        )
        .unwrap());
    } else {
        Err(rocket::response::status::Unauthorized::<String>(
            serde_json::to_string("You are not authorized.").unwrap(),
        ))
    }
}

#[derive(Debug)]
pub enum JwtErrorType {
    HmacError,
    JwtError,
    NoClaims,
}

fn generate_jwt(username: &str) -> Result<String, JwtErrorType> {
    dotenv().expect("Unable to load dotenv file.");
    let private_key = env::var("JWT_KEY").expect("Unable to load the JWT_KEY env variable.");

    let key: Hmac<Sha256> =
        Hmac::new_from_slice(&private_key.as_bytes()).expect("Unable to generate Hmac slice.");

    let mut claims = BTreeMap::new();
    claims.insert("username", username);
    let token_str = claims
        .sign_with_key(&key)
        .expect("Unable to generate token string.");

    Ok(token_str)
}

pub fn verify_jwt(token: &str, username: &str) -> Result<(), JwtErrorType> {
    dotenv().expect("Unable to load dotenv file.");
    let private_key = env::var("JWT_KEY").expect("Unable to load the JWT_KEY env variable.");

    let key: Hmac<Sha256> =
        Hmac::new_from_slice(&private_key.as_bytes()).expect("Unable to generate Hmac slice.");

    let token_str = token;
    let claims: BTreeMap<String, String> = match token_str.verify_with_key(&key) {
        Ok(claims) => claims,
        Err(_) => BTreeMap::new(),
    };
    println!("{:?}", claims);
    println!(
        "{:?}, {:?}",
        claims.get("username"),
        Some(&username.to_string())
    );

    if claims.get("username") == Some(&username.to_string()) {
        Ok(())
    } else {
        Err(JwtErrorType::NoClaims)
    }
}
