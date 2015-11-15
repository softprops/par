extern crate par;

fn main() {
    let bar = par::Bar::new(
        1024*4
    );
    for _ in 1..(1024 * 4) {
        bar.incr();
        bar.update();
        std::thread::sleep_ms(10);
    }
    bar.finish_print("done");
}
