pub mod prime_gen;
pub mod numbers;

extern crate num;

use std::time::Duration;

#[derive(Clone)]
pub struct ProblemResult {
    pub id : String,
    pub elapsed : Duration,
    pub result : String,
}

impl ProblemResult {
    pub fn new(id_arg : String, elapsed_arg : Duration, result_arg : String) -> ProblemResult {
        ProblemResult {
            id : id_arg,
            elapsed : elapsed_arg,
            result : result_arg,
        }
    }
}

#[macro_export]
macro_rules! problem {
    ($main_func:expr) => (
        use std::time::Duration;
        use std::time::Instant;

        use util::ProblemResult;

        pub fn solve() -> ProblemResult {
            let start : Instant = Instant::now();

            let result = $main_func();
            let dur : Duration = start.elapsed();

            ProblemResult::new(file!().to_string(), dur, result)
        }
    );

    ($main_func:expr, $right_answer:expr) => (
        use std::time::Duration;
        use std::time::Instant;

        use util::ProblemResult;

        #[test]
        pub fn test() {
            assert_eq!($main_func(), stringify!($right_answer).to_string());
        }

        pub fn solve() -> ProblemResult {
            let start : Instant = Instant::now();

            let result = $main_func();
            let dur : Duration = start.elapsed();

            assert_eq!(result, stringify!($right_answer).to_string());

            ProblemResult::new(file!().to_string(), dur, result)
        }
    );
}
