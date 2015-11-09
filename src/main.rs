extern crate par;

fn main() {
    let bar = par::Bar::new(100);
    for i in 1..101 {
        bar.write(i);
        std::thread::sleep_ms(20);
    }
}
