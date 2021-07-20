use crate::resources::{get_page_source, compute_hash, get_unix_time};
use crate::resources::{INDIVIDUAL_TIME_STAMP, INDIVIDUAL_STREAM_ID, CHANNEL_NAME, CLOUDFRONT_DOMAINS};

pub(crate) fn compute_vod(tracker_link: &str) {
    let page_source = get_page_source(tracker_link);

    let time_stamp = &INDIVIDUAL_TIME_STAMP
        .find(&page_source)
        .unwrap().as_str()[15..34];
    let unix_time = get_unix_time(time_stamp);
    let id = &INDIVIDUAL_STREAM_ID
        .find(tracker_link)
        .unwrap().as_str()[8..19];
    let channel_name = &CHANNEL_NAME
        .find(&page_source)
        .unwrap().as_str();
    let channel_name = &channel_name[7..channel_name.len() - 1];
    let body = format!("{}_{}_{}", channel_name, id, unix_time);

    let subdirectory = get_subdirectory(&body);
    //from: https://twitchtracker.com/scarly/streams/42517972332
    //prop: e5772c80744862fadae7_scarly_42517972332_1625247869
}

fn get_subdirectory(body: &str) -> String {
    let hash = compute_hash(body);
    format!("{}_{}", hash, body)
}

fn test_links(subdirectory: &str) {

}