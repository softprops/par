extern crate capsize;

use std::iter;
use std::sync::atomic::{AtomicUsize, Ordering};

static FORMAT: &'static str = "[=>_]";

pub struct Bar<'a> {
    prefix: &'a str,
    total: usize,
    current: AtomicUsize,
    format: &'a str
}

impl<'a> Bar<'a> {
    pub fn new(total: usize, prefix: Option<&'a str>) -> Bar<'a> {
        Bar {
            prefix: prefix.unwrap_or(""),
            total: total,
            current: AtomicUsize::new(0),
            format: FORMAT
        }
    }

    pub fn incr(&self) -> usize {
        self.add(1)
    }

    pub fn add(&self, delta: usize) -> usize {
        self.current.fetch_add(delta, Ordering::Relaxed)
    }

    pub fn width(&self) -> usize {
        80
    }

    pub fn update(&self) {
        let current = self.current.load(Ordering::Relaxed);
        self.write(current)
    }

    pub fn finish_print(&self, msg: &str) {
        println!("{}",msg)
    }

    fn repeat(what: &str, n: usize) -> String {
        iter::repeat(what).take(n).collect::<String>()
    }

    fn write(&self, current: usize) {
        let width = self.width();
        let prefix_display = self.prefix;

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
        let percent = current as f64 / (self.total as f64 / 100_f64);
        let percent_display = format!(
            " {:.*} %", 2, percent
        );
        let bar_width = format!(
            "{}{}{}{}{}", prefix_display, counter_display, percent_display, bar_start, bar_end
        ).chars().collect::<Vec<char>>().len();
        let size = width - bar_width;
        let cur_count = (
            (current as f64 / self.total as f64) * size as f64
        ).ceil() as usize;
        let empt_count = size - cur_count;
        bar_display.push_str(bar_start);
        bar_display.push_str(
            &Bar::repeat(current_marker, (cur_count - 1) as usize)
        );
        bar_display.push_str(current_n);
        bar_display.push_str(
            &Bar::repeat(empty, (empt_count) as usize)
        );
        bar_display.push_str(bar_end);
        print!("\r{}{}{}{}", prefix_display, counter_display, bar_display, percent_display)
    }
}

#[test]
fn it_works() {
}
