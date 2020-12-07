#[macro_use]
extern crate profiler;

mod fibonacci;
use fibonacci::fibonacci;

#[test]
fn threaded() {
    let hand1 = std::thread::spawn(|| {
        profiler_function!();
        fibonacci(40);
    });

    let hand2 = std::thread::spawn(|| {
        profiler_function!();
        fibonacci(40);
    });

    hand1.join().unwrap();
    hand2.join().unwrap();
}
