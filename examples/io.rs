extern crate hyper;
extern crate par;

use std::io::{self, Read};
use hyper::Client;
use hyper::header::{ContentLength, Connection, UserAgent};
use par::{Bar, Reader};

fn main() {
    let client = Client::new();
    let mut res = client.get("https://api.github.com/users/softprops/repos")
        .header(Connection::close())
        .header(UserAgent(String::from("par/0.1.0")))
        .send().unwrap();
    if let Some(&ContentLength(len)) = res.headers.clone().get::<ContentLength>() {
        let mut bar = Bar::new(len as usize);
        bar.units = par::Units::Bytes;
        let mut proxy = Reader::new(res, bar);
        let mut buf = String::new();
        proxy.read_to_string(&mut buf);
    };


}
