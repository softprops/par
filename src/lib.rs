extern crate termsize;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::io::Read;

pub struct ProgressReader<'a, R> {
    inner: R,
    bar: &'a Bar<'a>,
}

impl<'a, R: Read> ProgressReader<'a, R> {
    pub fn new(r: R, bar: &'a Bar<'a>) -> ProgressReader<'a, R> {
        ProgressReader { inner: r, bar: bar }
    }
}

impl<'a, R: Read> Read for ProgressReader<'a, R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let amt = try!(self.inner.read(buf));
        self.bar.add(amt);
        self.bar.update();
        return Ok(amt)
    }
}

pub struct Bar<'a> {
    prefix: Option<&'a str>,
    total: u64,
    current: AtomicUsize,
    format: &'a str
}

impl<'a> Bar<'a> {
    pub fn new(total: u64, prefix: Option<&'a str>) -> Bar<'a> {
        Bar {
            prefix: prefix,
            total: total,
            current: AtomicUsize::new(0),
            format: "[=>_]"
        }
    }

    pub fn incr(&self) -> usize {
        self.add(1)
    }

    pub fn add(&self, delta: usize) -> usize {
        self.current.fetch_add(delta, Ordering::Relaxed)
    }

    pub fn width(&self) -> u64 {
       termsize::get().unwrap().cols as u64
    }

    pub fn update(&self) {
        let current = self.current.load(Ordering::Relaxed);
        self.write(current as u64)
    }

    fn write(&self, current: u64) {
        let width = self.width();
        let prefix_display = self.prefix.clone().unwrap_or("");
        let formats = self.format.split("").collect::<Vec<&str>>();
        let bar_start = formats[1];
        let current_marker = formats[2];
        let current_n = formats[3];
        let empty = formats[4];
        let bar_end = formats[5];
        let mut bar_display = String::new();
        let counter_display = format!(
            "{} / {} ", current, self.total
        );
        let percent = current as f64 / (self.total as f64 / 100 as f64);
        let percent_display = format!(
            " {:.2} %", percent
        );
        let bar_width = format!(
            "{}{}{}{}{}", prefix_display, counter_display, percent_display, bar_start, bar_end
        ).chars().collect::<Vec<char>>().len() as u64;
        let size = width - bar_width;
        let cur_count = (
            (current as f64 / self.total as f64) * size as f64
        ).ceil() as u64;
        let empt_count = size - cur_count;
        bar_display.push_str(bar_start);
        bar_display.push_str(
            &String::from_utf8(
                vec![b'='; (cur_count - 1) as usize]
            ).unwrap()
        );
        bar_display.push_str(current_n);
        bar_display.push_str(
            &String::from_utf8(
                vec![b'-'; (empt_count) as usize]
            ).unwrap()
        );
        bar_display.push_str(bar_end);
        print!("\r{}{}{}{}", prefix_display, counter_display, bar_display, percent_display)
    }
}

#[test]
fn it_works() {
}
