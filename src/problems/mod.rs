use util::ProblemResult;

mod problem001;
mod problem002;
mod problem003;
mod problem004;
mod problem005;
mod problem006;
mod problem007;
mod problem008;
mod problem009;

mod problem010;
mod problem011;
mod problem012;
mod problem013;
mod problem014;
mod problem015;
mod problem016;
mod problem017;
mod problem018;
mod problem019;

mod problem020;
mod problem021;
mod problem022;
mod problem023;
mod problem024;
mod problem025;
mod problem026;
mod problem027;
mod problem028;
mod problem029;

mod problem030;
mod problem031;
mod problem032;
mod problem033;
mod problem034;
mod problem035;
mod problem036;
mod problem037;
mod problem038;
mod problem039;

mod problem040;
mod problem041;
mod problem042;
mod problem043;
mod problem044;
mod problem045;
mod problem046;
mod problem047;
mod problem048;
mod problem049;

mod problem050;
mod problem051;
mod problem052;
mod problem053;
mod problem054;
mod problem055;
mod problem056;
mod problem057;
mod problem058;
mod problem059;

mod problem060;
mod problem061;
mod problem062;
mod problem063;
mod problem064;
mod problem065;

pub fn solve_problem(i : usize) -> ProblemResult {
    let solvers : Vec<fn() -> ProblemResult> = vec![
problem001::solve,
problem002::solve,
problem003::solve,
problem004::solve,
problem005::solve,
problem006::solve,
problem007::solve,
problem008::solve,
problem009::solve,

problem010::solve,
problem011::solve,
problem012::solve,
problem013::solve,
problem014::solve,
problem015::solve,
problem016::solve,
problem017::solve,
problem018::solve,
problem019::solve,

problem020::solve,
problem021::solve,
problem022::solve,
problem023::solve,
problem024::solve,
problem025::solve,
problem026::solve,
problem027::solve,
problem028::solve,
problem029::solve,

problem030::solve,
problem031::solve,
problem032::solve,
problem033::solve,
problem034::solve,
problem035::solve,
problem036::solve,
problem037::solve,
problem038::solve,
problem039::solve,

problem040::solve,
problem041::solve,
problem042::solve,
problem043::solve,
problem044::solve,
problem045::solve,
problem046::solve,
problem047::solve,
problem048::solve,
problem049::solve,

problem050::solve,
problem051::solve,
problem052::solve,
problem053::solve,
problem054::solve,
problem055::solve,
problem056::solve,
problem057::solve,
problem058::solve,
problem059::solve,

problem060::solve,
problem061::solve,
problem062::solve,
problem063::solve,
problem064::solve,
problem065::solve
];
    
    let f : fn() -> ProblemResult = solvers[i];
    f()
}

