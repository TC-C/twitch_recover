use std::io;
use vod_core::{compute_vod};

mod resources;
mod vod_core;

fn main() { vod_recover_reader()}

fn vod_recover_reader() {
    println!("Enter a twitchtracker link that points to the stream >>> ");
    let mut twitchtracker_link = String::new();
    io::stdin()
        .read_line(&mut twitchtracker_link)
        .unwrap();
    twitchtracker_link = String::from(twitchtracker_link.trim_end_matches(&['\r', '\n'][..]));
    let vod_m3u8 = compute_vod(&twitchtracker_link);
}

