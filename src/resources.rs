use chrono::NaiveDateTime;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::blocking::get;
use std::io::Read;
use sha1::{Sha1, Digest};

lazy_static! {
    pub(crate) static ref GET_STREAM_ID: Regex = Regex::new("(data-stream=\").*?(\")").unwrap(); //13..24
    pub(crate) static ref GET_TIME_STAMP: Regex = Regex::new("(nowrap data-order=\").*?(\")").unwrap(); //19..38

    pub(crate) static ref INDIVIDUAL_TIME_STAMP: Regex = Regex::new("(<li><div><span>).*?(</span>)").unwrap(); //15..34
    pub(crate) static ref INDIVIDUAL_STREAM_ID: Regex = Regex::new("(streams/)\\d*$()").unwrap(); //8..19

    pub(crate) static ref CHANNEL_NAME: Regex = Regex::new("(name: ')[A-Za-z]*?(')").unwrap(); //7..len()-1
}

pub(crate) const CLOUDFRONT_DOMAINS: [&str; 8] = [
    "d2e2de1etea730", "dqrpb9wgowsf5", "ds0h3roq6wcgc",
    "d2nvs31859zcd8", "d2aba1wr3818hz", "d3c27h4odz752x",
    "dgeft87wbj63p", "d1m7jfoe9zdc1j"
];

pub(crate) fn get_page_source(url: &str) -> String {
    let mut source = String::new();
    get(url)
        .unwrap()
        .read_to_string(&mut source)
        .unwrap();
    source
}

pub(crate) fn compute_hash(body: &str) -> String {
    let mut hash = format!("{:x}", Sha1::digest(body.as_ref()));
    hash.truncate(20);
    hash
}

pub(crate) fn get_unix_time(time: &str) -> i64 {
    let utc_time = NaiveDateTime::parse_from_str(time, "%Y-%m-%d %H:%M:%S").unwrap();
    utc_time.timestamp()
}

/// Returns loc if a request can successfully be made
pub(crate) fn make_request(loc: &str) -> Result<String, ()> {
    let mut response = String::new();
    get(loc)
        .unwrap()
        .read_to_string(&mut response);
    if response.contains("Error") || response.contains("not found") {
        Err(())
    } else {
        Ok(String::from(loc))
    }
}
