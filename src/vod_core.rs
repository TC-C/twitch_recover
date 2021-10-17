use crate::resources;
use crate::resources::{get_page_source, get_unix_time};
use crate::resources::{CHANNEL_NAME, INDIVIDUAL_STREAM_ID, INDIVIDUAL_TIME_STAMP};

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
            return Err(format!(
                "Could not find a timestamp in the referenced link, Cloudflare may be blocking the page, try again later. Error: {}", page_source,
            ))
        }
        Some(time_stamp) => &time_stamp.as_str()[37..56],
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

    let subdirectory = resources::get_subdirectory(&body);
    resources::test_links(&subdirectory)
}
