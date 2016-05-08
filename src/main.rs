#[macro_use]
mod util;
mod problems;

use problems::Solver;
use util::ProblemResult;

use std::time::Duration;
use std::io;

fn main() {
    let mut solver = Solver::new();

    println!("Which problem do you want the answer to?");
    
    let mut pnum = String::new();
    
    io::stdin().read_line(&mut pnum)
        .ok()
        .expect("Failed to read line");
        
    let pnum : i32 = pnum.trim().parse()
        .ok()
        .expect("Please type a number!");

    if pnum == -1 {
        println!("Running all...");
        
        let mut total_time : Duration = Duration::from_secs(0);
        
        for r in solver {
            total_time = total_time + r.elapsed;
            println!("{} {}s {}ms : {}", r.id, r.elapsed.as_secs(), r.elapsed.subsec_nanos() / 1_000_000, r.result);
        }

        println!("Ran in {}s {}ms", total_time.as_secs(), total_time.subsec_nanos() / 1000000);
    } else {
        let result : Option<ProblemResult> = solver.solve_problem(pnum as usize);
        match result {
            Some(r) => {
                println!("{}:", r.id);
                println!("{}", r.result);
                println!("Ran in {}s ({}ms)", r.elapsed.as_secs(), r.elapsed.subsec_nanos() / 1000000);
            },
            None => println!("Problem not solved yet!"),
        }
        
    }


}