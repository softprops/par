extern crate par;

fn main() {
    let bar = par::Bar::new(
        100, Some("example ")
    );
    for _ in 1..101 {
        bar.incr();
        bar.update();
        std::thread::sleep_ms(10);
    }
}
