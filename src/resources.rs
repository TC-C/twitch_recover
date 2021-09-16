use std::io::{stdin, stdout, Read, Write};
use std::thread::{spawn, JoinHandle};

use chrono::NaiveDateTime;
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::blocking::Client;
use sha1::{Digest, Sha1};

lazy_static! {
    pub(crate) static ref GET_STREAM_ID: Regex = Regex::new("(data-stream=\").*?(\")").unwrap(); //13..24
    pub(crate) static ref GET_TIME_STAMP: Regex = Regex::new("(nowrap data-order=\").*?(\")").unwrap(); //19..38

    pub(crate) static ref INDIVIDUAL_TIME_STAMP: Regex = Regex::new("(<li><div><span>).*?(</span>)").unwrap(); //15..34
    pub(crate) static ref INDIVIDUAL_STREAM_ID: Regex = Regex::new("(streams/)\\d*$()").unwrap(); //8..19

    pub(crate) static ref CHANNEL_NAME: Regex = Regex::new(r#"name: '((#)?[a-zA-Z0-9][\w]{2,24})'"#).unwrap(); //7..len()-1

    static ref CLIENT: Client = Client::new();
}

pub(crate) const CLOUDFRONT_DOMAINS: [&str; 9] = [
    "d2e2de1etea730",
    "dqrpb9wgowsf5",
    "ds0h3roq6wcgc",
    "d2nvs31859zcd8",
    "d2aba1wr3818hz",
    "d3c27h4odz752x",
    "dgeft87wbj63p",
    "d1m7jfoe9zdc1j",
    "d1ymi26ma8va5x",
];
pub(crate) const VOD_DOMAINS: [&str; 3] = ["vod-secure", "vod-metro", "vod-pop-secure"];

pub(crate) fn get_page_source(url: &str) -> Result<String, String> {
    let mut source = String::new();
    match CLIENT.get(url).send() {
        Ok(mut response) => {
            match response.read_to_string(&mut source) {
                Ok(_) => {
                    let status = response.status();
                    if status.is_client_error() || status.is_server_error() {
                        return Err(status.to_string());
                    }
                }
                Err(e) => source = e.to_string(),
            }
            Ok(source)
        }
        Err(e) => Err(e.to_string()),
    }
}

pub(crate) fn compute_hash(body: &str) -> String {
    let mut hash = format!("{:x}", Sha1::digest(body.as_ref()));
    hash.truncate(20);
    hash
}

pub(crate) fn get_unix_time(time: &str) -> i64 {
    NaiveDateTime::parse_from_str(time, "%Y-%m-%d %H:%M:%S")
        .unwrap()
        .timestamp()
}

pub(crate) fn error(message: &str) {
    execute!(
        stdout(),
        SetForegroundColor(Color::Red),
        Print(format!("\n{}", message)),
        ResetColor
    )
        .unwrap();
}

pub(crate) fn ask(message: &str) -> String {
    let mut response = String::new();
    print!("{}", message);
    stdout().flush().unwrap();
    stdin().read_line(&mut response).unwrap();
    response.trim_end_matches(&['\r', '\n'][..]).to_owned()
}

pub fn test_links(subdirectory: &str) -> Result<String, String> {
    let mut threads: Vec<(String, JoinHandle<Result<String, String>>)> =
        Vec::with_capacity(CLOUDFRONT_DOMAINS.len() + VOD_DOMAINS.len());
    for domain in CLOUDFRONT_DOMAINS {
        let url = format!(
            "https://{}.cloudfront.net/{}/chunked/index-dvr.m3u8",
            domain, subdirectory
        );
        let url_copy = url.to_owned();
        let thread = spawn(move || get_page_source(&url_copy));
        threads.push((url, thread));
    }
    for domain in VOD_DOMAINS {
        let url = format!(
            "https://{}.twitch.tv/{}/chunked/index-dvr.m3u8",
            domain, subdirectory
        );
        let url_copy = url.to_owned();
        let thread = spawn(move || get_page_source(&url_copy));
        threads.push((url, thread));
    }

    for thread in threads {
        let url = &thread.0;
        let thread = thread.1;
        if thread.join().unwrap().is_ok() {
            return Ok(url.to_owned());
        }
    }
    Err(String::from("No available M3U8 URLs"))
}

pub fn get_subdirectory(body: &str) -> String {
    let hash = compute_hash(body);
    format!("{}_{}", hash, body)
}
