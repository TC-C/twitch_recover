use crate::channel_core::compute_channel;
use crate::resources::{ask, error};
use vod_core::compute_vod;

//mod channel_core;
mod channel_core;
mod resources;
mod vod_core;

fn main() {
    let input = ask("Would you like to recover a single VOD or an entire channel [VOD, Channel]: ")
        .to_lowercase();
    let input = input.as_str();
    match input {
        "vod" => vod_recover_reader(),
        "channel" => channel_recover_reader(),
        &_ => {}
    }
}

fn vod_recover_reader() {
    let twitchtracker_link = ask("Enter a twitchtracker link that points to the stream >>> ");
    let twitchtracker_link = twitchtracker_link.as_str();
    match compute_vod(twitchtracker_link) {
        Ok(link) => println!("{}", link),
        Err(e) => error(&e),
    }
}

fn channel_recover_reader() {
    let channel_name = ask("Enter a channel name to recover >>> ");
    compute_channel(&channel_name);
}
