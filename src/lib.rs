extern crate termsize;

use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};

pub struct Bar {
    total: u64,
    current: u64,
    sender: Sender<u64>
}

impl Bar {
    pub fn new(total: u64) -> Bar {
        let (tx, rx) = channel();
        Bar {
            total: total,
            current: 0,
            sender: tx
        }
    }

    pub fn width(&self) -> u64 {
       termsize::get().unwrap().cols as u64
    }

    pub fn write(&self, current: u64) {
        let width = self.width();
        let mut bar_display = String::new();
        let counter_display = format!(
            "{} / {} ", current, self.total
        );
        let percent = current as f64 / (self.total as f64 / 100 as f64);
        let percent_display = format!(
            " {:.2} %", percent
        );
        let bar_width = format!(
            "{}{}[]",counter_display, percent_display
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
        print!("\r{}{}{}", counter_display, bar_display, percent_display)
    }
}

#[test]
fn it_works() {
}
