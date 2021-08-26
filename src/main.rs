use crate::resources::{ask, error};
use vod_core::compute_vod;

mod resources;
mod vod_core;

fn main() {
    vod_recover_reader()
}

fn vod_recover_reader() {
    let twitchtracker_link = ask("Enter a twitchtracker link that points to the stream >>> ");
    let twitchtracker_link = twitchtracker_link.trim_end_matches(&['\r', '\n'][..]);
    match compute_vod(twitchtracker_link) {
        Ok(link) => println!("{}", link),
        Err(e) => error(&e),
    }
}

fn channel_recover_reader() {
    let channel_name = ask("Enter a channel name to recover >>> ");
}
