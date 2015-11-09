extern crate termsize;

use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Bar {
    prefix: Option<String>,
    total: u64,
    current: AtomicUsize
}

impl Bar {
    pub fn new(total: u64, prefix: Option<&str>) -> Bar {
        Bar {
            prefix: prefix.clone().map(|s|s.to_owned()),
            total: total,
            current: AtomicUsize::new(0)
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
        self.write(self.current.load(Ordering::Relaxed) as u64)
    }

    fn write(&self, current: u64) {
        let width = self.width();
        let prefix_display = self.prefix.clone().unwrap_or("".to_owned());
        let mut bar_display = String::new();
        let counter_display = format!(
            "{} / {} ", current, self.total
        );
        let percent = current as f64 / (self.total as f64 / 100 as f64);
        let percent_display = format!(
            " {:.2} %", percent
        );
        let bar_width = format!(
            "{}{}{}[]", prefix_display, counter_display, percent_display
        ).chars().collect::<Vec<char>>().len() as u64;
        let size = width - bar_width;
        let cur_count = (
            (current as f64 / self.total as f64) * size as f64
        ).ceil() as u64;
        let empt_count = size - cur_count;
        bar_display.push_str("[");
        bar_display.push_str(
            &String::from_utf8(
                vec![b'='; (cur_count - 1) as usize]
            ).unwrap()
        );
        bar_display.push_str(">");
        bar_display.push_str(
            &String::from_utf8(
                vec![b'_'; (empt_count) as usize]
            ).unwrap()
        );
        bar_display.push_str("]");
        print!("\r{}{}{}{}", prefix_display, counter_display, bar_display, percent_display)
    }
}

#[test]
fn it_works() {
}
