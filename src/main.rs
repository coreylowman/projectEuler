#[macro_use]
mod util;
mod problems;

use util::ProblemResult;
use std::time::Duration;

fn main() {
    let mut results : Vec<ProblemResult> = Vec::new();
    results.push(problems::solve_problem(0));

    let mut totalTime : Duration = Duration::from_secs(0);


}