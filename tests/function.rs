#[macro_use]
extern crate profiler;

mod fibonacci;
use fibonacci::fibonacci;

#[test]
fn function() {
    profiler_function!();
    fibonacci(40);
}
