use std::env;
use std::env::VarError;

#[derive(Debug)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

impl Credentials {

    #[allow(dead_code)]
    pub fn from_env() -> Result<Credentials, VarError> {
        let email = env::var("ANCHOR_EMAIL")?;
        let password = env::var("ANCHOR_PASSWORD")?;
        Ok(Credentials { email, password })
    }
}
