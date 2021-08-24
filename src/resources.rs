use chrono::NaiveDateTime;
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::blocking::Client;
use sha1::{Digest, Sha1};
use std::io::{self, stdout, Read, Write};

lazy_static! {
    pub(crate) static ref GET_STREAM_ID: Regex = Regex::new("(data-stream=\").*?(\")").unwrap(); //13..24
    pub(crate) static ref GET_TIME_STAMP: Regex = Regex::new("(nowrap data-order=\").*?(\")").unwrap(); //19..38

    pub(crate) static ref INDIVIDUAL_TIME_STAMP: Regex = Regex::new("(<li><div><span>).*?(</span>)").unwrap(); //15..34
    pub(crate) static ref INDIVIDUAL_STREAM_ID: Regex = Regex::new("(streams/)\\d*$()").unwrap(); //8..19

    pub(crate) static ref CHANNEL_NAME: Regex = Regex::new("(name: ')[A-Za-z0-9]*?(')").unwrap(); //7..len()-1

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
                    if status.as_u16() >= 400 {
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
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut response).unwrap();
    response
}
