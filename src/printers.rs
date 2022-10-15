use crate::anchor_client::*;

use serde::{Deserialize, Serialize};
use ureq::serde_json;

#[derive(Serialize, Deserialize, Clone)]
struct Episodes {
    episodes: Vec<Episode>,
}

pub struct Printers {}

impl Printers {
    pub fn print_csv(episodes: &Vec<Episode>) -> () {
        for episode in episodes.iter() {
            println!("\"{}\",\"{}\"", episode.title, episode.totalPlays)
        }
    }

    pub fn print_json(episodes: &Vec<Episode>) -> () {
        println!(
            "{}",
            serde_json::to_string(&Episodes {
                episodes: episodes.clone()
            })
            .expect("Problem with serialization to JSON.")
        );
    }

    pub fn print_string(episodes: &Vec<Episode>) -> () {
        for episode in episodes.iter() {
            println!("{} {}", episode.title, episode.totalPlays)
        }
    }
}
