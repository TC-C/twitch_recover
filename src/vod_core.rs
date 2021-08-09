use crate::resources::{compute_hash, get_page_source, get_unix_time};
use crate::resources::{
    CHANNEL_NAME, CLOUDFRONT_DOMAINS, INDIVIDUAL_STREAM_ID, INDIVIDUAL_TIME_STAMP, VOD_DOMAINS,
};
use std::thread::{spawn, JoinHandle};

pub(crate) fn compute_vod(tracker_link: &str) -> Result<String, String> {
    let page_source = match get_page_source(tracker_link) {
        Ok(source) => source,
        Err(e) => return Err(e),
    };
    let id = match &INDIVIDUAL_STREAM_ID.find(tracker_link) {
        None => return Err(format!("Could not find the Stream ID in {}", tracker_link)),
        Some(id) => &id.as_str()[8..19],
    };

    let time_stamp = match &INDIVIDUAL_TIME_STAMP.find(&page_source) {
        None => {
            return Err(String::from(
                "Could not find a timestamp in the referenced link, Cloudflare may be blocking the page, try again later",
            ))
        }
        Some(time_stamp) => &time_stamp.as_str()[15..34],
    };
    let unix_time = get_unix_time(time_stamp);
    let channel_name = match CHANNEL_NAME.find(&page_source) {
        None => {
            return Err(String::from(
                "Could not find channel name in the referenced link",
            ))
        }
        Some(channel_name) => channel_name.as_str(),
    };

    let channel_name = &channel_name[7..channel_name.len() - 1];
    let body = format!("{}_{}_{}", channel_name, id, unix_time);

    let subdirectory = get_subdirectory(&body);
    match test_links(&subdirectory) {
        Ok(link) => Ok(link),
        Err(e) => Err(e),
    }
}

fn get_subdirectory(body: &str) -> String {
    let hash = compute_hash(body);
    format!("{}_{}", hash, body)
}

fn test_links(subdirectory: &str) -> Result<String, String> {
    let mut threads: Vec<JoinHandle<Result<String, String>>> =
        Vec::with_capacity(CLOUDFRONT_DOMAINS.len());
    for domain in CLOUDFRONT_DOMAINS {
        let url = format!(
            "https://{}.cloudfront.net/{}/chunked/index-dvr.m3u8",
            domain, subdirectory
        );
        let thread = spawn(move || get_page_source(&url));
        threads.push(thread);
    }
    for domain in VOD_DOMAINS {
        let url = format!(
            "https://{}.twitch.tv/{}/chunked/index-dvr.m3u8",
            domain, subdirectory
        );
        let thread = spawn(move || get_page_source(&url));
        threads.push(thread);
    }

    for thread in threads {
        if let Ok(url) = thread.join().unwrap() {
            return Ok(url);
        }
    }
    Err(String::from("No available M3U8 URLs"))
}
