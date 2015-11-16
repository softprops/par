extern crate par;

fn main() {
    let mut bar = par::Bar::new(
        100
   );
    bar.prefix = String::from("dl ");
    for _ in 1..101 {
        bar.incr();
        std::thread::sleep_ms(10);
    }
    bar.finish_print("done");
}
