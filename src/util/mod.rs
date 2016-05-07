pub mod prime_gen;

extern crate num;

use std::time::Duration;
use std::time::Instant;

#[derive(Clone)]
pub struct ProblemResult {
    pub elapsed : Duration,
    pub result : String
}

impl ProblemResult {
    pub fn new(elapsedArg : Duration, resultArg : String) -> ProblemResult {
        ProblemResult {
            elapsed : elapsedArg,
            result : resultArg,
        }
    }
}

#[macro_export]
macro_rules! problem {
    ($f:expr) => (
        use std::time::Duration;
        use std::time::Instant;

        use util::ProblemResult;

        pub fn solve() -> ProblemResult {
            let start : Instant = Instant::now();

            let result = $f();
            let dur : Duration = start.elapsed();

            ProblemResult::new(dur, result)
        }
    );
}
