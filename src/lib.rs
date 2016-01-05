//! par is a terminal interface for rendering progress bars
//!
//! #example
//! ```rust
//! use par::Bar;
//! fn main() {
//!     let bar = Bar::new(100);
//!     for i in 1..101 {
//!         bar.add(i);
//!         std::thread::sleep_ms(10);
//!     }
//!     bar.finish_print("done")
//! }
//! ```
extern crate capsize;
extern crate termsize;

use capsize::Capacity;
use std::fmt;
use std::io::{self, Read, Write};
use std::iter;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::default::Default;

static FORMAT: &'static str = "[=>_]";

/// Unit of display for progress counter
#[derive(Debug)]
pub enum Units {
    /// No units
    None,
    /// format progress in humanized byte sizes
    Bytes
}

impl Default for Units {
    fn default() -> Units {
        Units::None
    }
}

/// Preference for reporting progress
#[derive(Debug)]
pub enum Reporter {
    /// Report progress to stdout
    StdOut,
    /// Report progress to stderr
    StdErr,
    /// Report progress to a callback
    Callback(fn(String) -> ()),
    /// Skip reporting
    None
}

impl Default for Reporter {
    fn default() -> Reporter {
        Reporter::StdErr
    }
}

/// write progress. Implementations for Read and Write are provided
pub struct Writer<T> {
    inner: T,
    bar: Bar
}

/// read progress. Implementations for Read and Write are provided
pub struct Reader<T> {
    inner: T,
    bar: Bar
}

impl <R: Read> Reader<R> {
    pub fn new(read: R, bar: Bar) -> Reader<R> {
        Reader {
            inner: read,
            bar: bar
        }
    }
}

impl <R: Read> Read for Reader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = try!(self.inner.read(buf));
        self.bar.add(n);
        Ok(n)
    }
}

impl <W: Write> Writer<W> {
    pub fn new(write: W, bar: Bar) -> Writer<W> {
        Writer {
            inner: write,
            bar: bar
        }
    }
}

impl <W: Write> Write for Writer<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let n = buf.len();
        self.bar.add(n);
        Ok(n)
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// A bar of progress information
#[derive(Debug, Default)]
pub struct Bar {
    total: usize,
    /// show progress percent, defaults to true
    pub show_percent: bool,
    /// show progress counter. Defaults to true
    pub show_counter: bool,
    /// show main progress bar UI. Defaults to true
    pub show_bar: bool,
    /// an arbitrary string label to prefix display with
    pub prefix: String,
    /// preference for reporting progress
    pub reporter: Reporter,
    current: AtomicUsize,
    pub units: Units,
    bar_start: String,
    bar_current: String,
    bar_current_n: String,
    bar_empty: String,
    bar_end: String
}

impl Bar {
    /// creates a new bar with a target total size of `total`
    pub fn new(total: usize) -> Bar {
        let mut bar = Bar {
            total: total,
            show_percent: true,
            show_counter: true,
            show_bar: true,
            ..Default::default()
        };
        bar.format(FORMAT);
        bar
    }

    /// sets a custom format for display
    /// Example: bar.format("[=>_]")
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
        let prev = self.current.fetch_add(delta, Ordering::Relaxed);
        self.update();
        prev
    }

    /// sets the bar count to specified value
    pub fn set(&self, value: usize) {
        self.current.store(value, Ordering::Relaxed);
        self.update()
    }

    pub fn update(&self) {
        let current = self.current.load(Ordering::Relaxed);
        if current <= self.total {
            self.progress();
        }
    }

    pub fn finish_print(&self, msg: &str) {
        println!("");
        println!("{}", msg)
    }

    fn width(&self) -> usize {
        termsize::get().map(|s|s.cols as usize).unwrap_or(80)
    }


    #[inline]
    pub fn percent_completed(&self) -> f64 {
        self.current.load(Ordering::Relaxed) as f64 / (self.total as f64 / 100_f64)
    }

    #[inline]
    fn percent_completed_str(&self) -> String {
        format!(
            " {:.*} %", 2, self.percent_completed()
        )
    }

    #[inline]
    fn counter_str(&self) -> String {
        fn unit(value: usize, units: &Units) -> String {
            match *units {
                Units::None => value.to_string(),
                Units::Bytes => (value as i64).capacity()
            }
        }
        let current = self.current.load(Ordering::Relaxed);
        format!(
            "{} / {} ", unit(current, &self.units), unit(self.total, &self.units)
        )
    }

    #[inline]
    fn to_str(&self) -> String {
        fn repeat(what: &str, n: usize) -> String {
            iter::repeat(what).take(n).collect::<String>()
        }

        let current = self.current.load(Ordering::Relaxed);
        let width = self.width();

        let mut prefix = self.prefix.clone();
        let mut mid = String::new();
        let mut suffix = String::new();

        // counter
        if self.show_counter {
            prefix = prefix + &self.counter_str();
        }

        // percent complete
        if self.show_percent {
            suffix = suffix + &self.percent_completed_str();
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
        display
    }

    fn progress(&self) {
        let display = self.to_str();
        match self.reporter {
            Reporter::StdErr => {
                let _ = write!(&mut std::io::stderr(), "\r{}", display);
            },
            Reporter::StdOut => {
                let _ = write!(&mut std::io::stderr(), "\r{}", display);
            },
            Reporter::Callback(cb) => {
                cb(display);
            },
            Reporter::None => ()
        }
    }
}

impl fmt::Display for Bar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut bar = Bar::new(100);
        bar.reporter = Reporter::None;
        bar.add(1);
        assert_eq!(bar.percent_completed(), 1_f64);
    }
}
