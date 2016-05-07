pub mod prime_gen;

extern crate num;

#[macro_export]
macro_rules! problem {
    ($f:expr) => (
        use std::time::Duration;
        use std::time::Instant;

        fn main() {
            println!("{}:", file!());
            
            let start : Instant = Instant::now();

            println!("{}", $f());
            
            let dur : Duration = start.elapsed();

            println!("Ran in {}s ({}ms)", dur.as_secs(), dur.subsec_nanos() / 1000000);
        }
    );
}
