use crate::anchor_client::*;
use crate::credentials::Credentials;
use crate::printers::Printers;
use clap::{Parser, ValueEnum};
use std::time::Duration;

mod anchor_client;
mod config;
mod credentials;
mod error;
mod printers;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Format {
    String,
    CSV,
    InfluxDBCSV,
    Json,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    #[arg(short, long, env = "ANCHOR_EMAIL")]
    email: String,
    #[arg(short, long, env = "ANCHOR_PASSWORD")]
    password: String,
    #[arg(short, long, value_enum, default_value_t = Format::String)]
    format: Format,
}

pub(crate) fn main() {
    let args: CliArgs = CliArgs::parse();

    let mut anchor = AnchorClient::from_agent(
        ureq::AgentBuilder::new()
            .timeout_read(Duration::from_secs(2))
            .timeout_write(Duration::from_secs(2))
            .build(),
    );

    let token = anchor.get_csrf_token().expect("Failed fetching token");
    anchor
        .post_login(
            &Credentials {
                email: args.email,
                password: args.password,
            },
            &token,
        )
        .expect("Login procedure has failed.");

    let episodes: Vec<Episode> = anchor
        .get_metadata()
        .and_then(|metadata| anchor.all_episodes(&metadata.webStationId))
        .expect("Failed fetching episodes.");

    match args.format {
        Format::CSV => Printers::print_csv(&episodes),
        Format::InfluxDBCSV => Printers::print_influx_csv(&episodes),
        Format::Json => Printers::print_json(&episodes),
        _ => Printers::print_string(&episodes),
    }
}
