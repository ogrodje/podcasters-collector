use serde::Deserialize;
use ureq::Error;

type IP = String;

#[derive(Deserialize)]
struct ServiceResponse {
    ip: IP,
}

fn get_my_ip() -> Result<String, Error> {
    let response: ServiceResponse = ureq::get("https://api.ipify.org/?format=json")
        .call()?
        .into_json()?;
    Ok(response.ip)
}

pub fn main() {
    let response = get_my_ip();
    let response = match response {
        Ok(ip) => ip,
        Err(e) => panic!("Boom! {e}"),
    };

    println!("{response}");
}
