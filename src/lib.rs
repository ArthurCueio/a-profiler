use std::time;

#[macro_export]
macro_rules! profiler_function {
    () => {
    let name ={
        // Very nice trick to get the name of the function I found here:
        // https://github.com/Vlad2001MFS/rprofiler/blob/3939cd1e06b980a1083cdbb53c16989db63cc1ad/src/lib.rs#L78
        fn f() {}

        fn type_name_of_val<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }type_name_of_val(f)
        };
        $crate::profiler_scope!(&name[..name.len() - 3]);
    };
}

#[macro_export]
macro_rules! profiler_scope {
    ($name:expr) => {
        let __profiler_scope = $crate::Scope::new($name);
    };
}

pub struct Scope {
    start_time: time::Instant,
    name: &'static str,
}

impl Scope {
    pub fn new(name: &'static str) -> Self {
        Scope {
            start_time: time::Instant::now(),
            name,
        }
    }
}

impl Drop for Scope {
    fn drop(&mut self) {
        let duration = time::Instant::now().duration_since(self.start_time);
        println!("{}: {}ms", self.name, duration.as_millis());
    }
}
