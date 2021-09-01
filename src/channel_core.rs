use crate::resources::{get_page_source, get_subdirectory, get_unix_time, test_links};
use crate::resources::{GET_STREAM_ID, GET_TIME_STAMP};

pub(crate) fn compute_channel(channel_name: &str) {
    let page_source = get_page_source(&format!(
        "https://twitchtracker.com/{}/streams",
        channel_name
    ))
    .unwrap();
    let ids = GET_STREAM_ID
        .find_iter(&page_source)
        .map(|i| &i.as_str()[13..24]);
    let unix_timestamps = GET_TIME_STAMP
        .find_iter(&page_source)
        .map(|t| get_unix_time(&t.as_str()[19..38]));
    let mut ids_and_ts: Vec<(&str, i64)> = ids.zip(unix_timestamps).collect();
    ids_and_ts.reverse();
    //dbg!(&ids_and_ts);
    for e in ids_and_ts {
        let id = e.0;
        let unix_time = e.1;
        let body = format!("{}_{}_{}", channel_name, id, unix_time);
        let subdirectory = get_subdirectory(&body);
        if let Ok(link) = test_links(&subdirectory) {
            println!("{}", link);
        }
    }
}
