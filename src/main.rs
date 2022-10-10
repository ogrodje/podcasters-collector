use std::time::Duration;

use anchor_client::AnchorClient;

use crate::credentials::Credentials;

mod anchor_client;
mod config;
mod credentials;
mod error;

pub(crate) fn main() {
    let credentials = Credentials::from_env().expect("Missing credentials.");

    let mut anchor = AnchorClient::from_agent(
        ureq::AgentBuilder::new()
            .timeout_read(Duration::from_secs(2))
            .timeout_write(Duration::from_secs(2))
            .build(),
    );

    let _login = anchor
        .get_csrf_token()
        .and_then(|token| anchor.post_login(&credentials, &token))
        .expect("Login procedure has failed.");

    let episodes = anchor
        .get_metadata()
        .and_then(|metadata| anchor.all_episodes(&metadata.webStationId))
        .expect("Failed fetching episodes.");

    for episode in episodes.iter() {
        println!("{} {}", episode.title, episode.totalPlays);
    }
}
