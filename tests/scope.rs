#[macro_use]
extern crate profiler;

mod fibonacci;
use fibonacci::fibonacci;

#[test]
fn test_scope() {
    {
        profiler_scope!("scope 1");
        fibonacci(40);
    }

    {
        profiler_scope!("scope 2");
        fibonacci(35);
    }
}
