use std::env;

#[derive(Debug)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

pub fn from_env() -> Credentials {
    Credentials {
        email: env::var("ANCHOR_EMAIL").expect("$ANCHOR_EMAIL is not set!"),
        password: env::var("ANCHOR_PASSWORD").expect("$ANCHOR_PASSWORD is not set!"),
    }
}
