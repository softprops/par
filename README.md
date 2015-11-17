# par

> a flexible progress bar for rust terminal interfaces

(heavily inspired by golang's [pb](https://github.com/cheggaaa/pb))

## api docs

Find them [here](https://softprops.github.io/par)

## usage

Basic usage requires the creation of a `par::Bar` struct with a target value

```rust
use par::Bar;

fn main() {
    let bar = Bar::new(100);
    for i in 1..101 {
        bar.add(i);
        std::thread::sleep_ms(10);
    }
    bar.finish_print("done");
}
```

Doug Tangren (softprops) 2015
