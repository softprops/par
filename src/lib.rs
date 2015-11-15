extern crate capsize;

use capsize::Capacity;
use std::iter;
use std::sync::atomic::{AtomicUsize, Ordering};


static FORMAT: &'static str = "[=>_]";

pub enum Units {
    None,
    Bytes
}

pub struct Bar {
    prefix: String,
    total: usize,
    current: AtomicUsize,
    units: Units,
    bar_start: String,
    bar_current: String,
    bar_current_n: String,
    bar_empty: String,
    bar_end: String,
    show_percent: bool,
    show_counter: bool,
    show_bar: bool
}

impl Bar {
    pub fn new(total: usize) -> Bar {
        let mut bar = Bar {
            prefix: String::new(),
            total: total,
            current: AtomicUsize::new(0),
            units: Units::Bytes,
            bar_start: String::new(),
            bar_current: String::new(),
            bar_current_n: String::new(),
            bar_empty: String::new(),
            bar_end: String::new(),
            show_percent: true,
            show_counter: true,
            show_bar: true
        };
        bar.format(FORMAT);
        bar
    }

    /// bar.format("[=>_]")
    pub fn format(&mut self, spec: &str) {
        if spec.len() == 5 {
            let parts = spec.split("").collect::<Vec<&str>>();
            self.bar_start = parts[1].to_owned();
            self.bar_current = parts[2].to_owned();
            self.bar_current_n = parts[3].to_owned();
            self.bar_empty = parts[4].to_owned();
            self.bar_end = parts[5].to_owned();
        }
    }

    /// increment bar count by one
    pub fn incr(&self) -> usize {
        self.add(1)
    }

    /// add 1 to the bar count
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

    fn write(&self, current: usize) {
        fn repeat(what: &str, n: usize) -> String {
            iter::repeat(what).take(n).collect::<String>()
        }

        fn unit(value: usize, units: &Units) -> String {
            match *units {
                Units::None => value.to_string(),
                Units::Bytes => (value as i64).capacity()
            }
        }

        let width = self.width();

        let mut prefix = String::new();
        let mut mid = String::new();
        let mut suffix = String::new();

        // counter
        if self.show_counter {
            prefix = format!(
                "{} / {} ", unit(current, &self.units), unit(self.total, &self.units)
            );
        }

        // percent complete
        if self.show_percent {
            let percent = current as f64 / (self.total as f64 / 100_f64);
            suffix = suffix + &format!(
                " {:.*} %", 2, percent
            );
        }

        if self.show_bar {
            let size = width - (prefix.len() + suffix.len() + 3);
            if size > 0 {
                let cur_count = (
                    (current as f64 / self.total as f64) * size as f64
                ).ceil() as usize;
                let empt_count = size - cur_count;
                mid = self.bar_start.clone();
                if empt_count > 0 {
                    mid = mid + &repeat(self.bar_current.as_ref(), (cur_count - 1) as usize) +  &self.bar_current_n;
                } else {
                    mid = mid + &repeat(self.bar_current.as_ref(), cur_count as usize);
                }
                mid = mid + &repeat(self.bar_empty.as_ref(), (empt_count) as usize) + &self.bar_end
            }
        }
        let mut display = prefix + &mid + &suffix;
        if display.len() < width {
            let remaining = width - display.len();
            display = display + &repeat(" ", remaining);
        }

        print!("\r{}", display)
    }
}

#[test]
fn it_works() {
}
