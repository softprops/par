extern crate par;
extern crate hyper;
use std::io::Read;

fn main() {
    /*let bar = par::Bar::new(
        100, Some("example ")
    );
    for _ in 1..101 {
        bar.incr();
        bar.update();
        std::thread::sleep_ms(10);
    }*/

    let client = hyper::Client::new();
    let mut res = client.get("https://github.com/").send().unwrap();
    let mut buffer = String::new();
    let bar = par::Bar::new(16763, Some("dl "));
    let mut reader = par::ProgressReader::new(res, &bar);
    reader.read_to_string(&mut buffer);
}
