use lazy_static::lazy_static;
use regex::Regex;
use std::io::Read;
use sha1::{Sha1, Digest};

lazy_static! {
    pub(crate) static ref GET_STREAM_ID: Regex = Regex::new("(data-stream=\").*?(\")").unwrap(); //13..24
    pub(crate) static ref GET_TIME_STAMP: Regex = Regex::new("(nowrap data-order=\").*?(\")").unwrap(); //19..38

    pub(crate) static ref INDIVIDUAL_TIME_STAMP: Regex = Regex::new("(<li><div><span>).*?(</span>)").unwrap(); //15..34
    pub(crate) static ref INDIVIDUAL_STREAM_ID: Regex = Regex::new("(streams/)\\d*$()").unwrap(); //8..19

    pub(crate) static ref CHANNEL_NAME: Regex = Regex::new("(name: ')[A-Za-z]*?(')").unwrap(); //9..19
}

pub(crate) fn get_page_source(url: &str) -> String {
    let mut source = String::new();
    reqwest::blocking::get(url)
        .unwrap()
        .read_to_string(&mut source)
        .unwrap();
    source
}

pub(crate) fn compute_hash(body: &str) -> String {
    let mut hash = Sha1::digest(body.as_ref()).to_string();
    hash.truncate(20);
    hash
}