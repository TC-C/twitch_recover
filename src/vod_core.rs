use crate::resources::{get_page_source, compute_hash};
use crate::resources::{INDIVIDUAL_TIME_STAMP, INDIVIDUAL_STREAM_ID, CHANNEL_NAME};

pub(crate) fn compute_vod(tracker_link: &str) {
    let page_source = get_page_source(tracker_link);

    let time_stamp = &INDIVIDUAL_TIME_STAMP
        .find(&page_source)
        .unwrap().as_str()[15..34];
    let id = &INDIVIDUAL_STREAM_ID
        .find(tracker_link)
        .unwrap().as_str()[8..9];
    let channel_name = CHANNEL_NAME
        .find(&page_source)
        .unwrap().as_str();
    let channel_name = &channel_name[7..channel_name.len()];

    let body = format!("{}_{}_{}", channel_name, id, time_stamp);

    let subdirectory = get_subdirectory(&body);
}

fn get_subdirectory(body: &str) -> String{
    let hash = compute_hash(body);
    format!("{}_{}", hash, body)
}