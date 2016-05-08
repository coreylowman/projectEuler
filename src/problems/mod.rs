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
mod problem066;
mod problem067;
mod problem068;

pub struct Solver {
    pub current : usize,
}

impl Solver {
    pub fn new() -> Solver {
        Solver {
            current : 1
        }
    }

    pub fn solve_problem(&mut self, i : usize) -> Option<ProblemResult> {
        match i {
            001 => Some(problem001::solve()),
            002 => Some(problem002::solve()),
            003 => Some(problem003::solve()),
            004 => Some(problem004::solve()),
            005 => Some(problem005::solve()),
            006 => Some(problem006::solve()),
            007 => Some(problem007::solve()),
            008 => Some(problem008::solve()),
            009 => Some(problem009::solve()),

            010 => Some(problem010::solve()),
            011 => Some(problem011::solve()),
            012 => Some(problem012::solve()),
            013 => Some(problem013::solve()),
            014 => Some(problem014::solve()),
            015 => Some(problem015::solve()),
            016 => Some(problem016::solve()),
            017 => Some(problem017::solve()),
            018 => Some(problem018::solve()),
            019 => Some(problem019::solve()),

            020 => Some(problem020::solve()),
            021 => Some(problem021::solve()),
            022 => Some(problem022::solve()),
            023 => Some(problem023::solve()),
            024 => Some(problem024::solve()),
            025 => Some(problem025::solve()),
            026 => Some(problem026::solve()),
            027 => Some(problem027::solve()),
            028 => Some(problem028::solve()),
            029 => Some(problem029::solve()),

            030 => Some(problem030::solve()),
            031 => Some(problem031::solve()),
            032 => Some(problem032::solve()),
            033 => Some(problem033::solve()),
            034 => Some(problem034::solve()),
            035 => Some(problem035::solve()),
            036 => Some(problem036::solve()),
            037 => Some(problem037::solve()),
            038 => Some(problem038::solve()),
            039 => Some(problem039::solve()),

            040 => Some(problem040::solve()),
            041 => Some(problem041::solve()),
            042 => Some(problem042::solve()),
            043 => Some(problem043::solve()),
            044 => Some(problem044::solve()),
            045 => Some(problem045::solve()),
            046 => Some(problem046::solve()),
            047 => Some(problem047::solve()),
            048 => Some(problem048::solve()),
            049 => Some(problem049::solve()),

            050 => Some(problem050::solve()),
            051 => Some(problem051::solve()),
            052 => Some(problem052::solve()),
            053 => Some(problem053::solve()),
            054 => Some(problem054::solve()),
            055 => Some(problem055::solve()),
            056 => Some(problem056::solve()),
            057 => Some(problem057::solve()),
            058 => Some(problem058::solve()),
            059 => Some(problem059::solve()),

            060 => Some(problem060::solve()),
            061 => Some(problem061::solve()),
            062 => Some(problem062::solve()),
            063 => Some(problem063::solve()),
            064 => Some(problem064::solve()),
            065 => Some(problem065::solve()),
            066 => Some(problem066::solve()),
            067 => Some(problem067::solve()),
            068 => Some(problem068::solve()),
            _ => None
        }
    }
}

impl Iterator for Solver {
    type Item = ProblemResult;

    fn next(&mut self) -> Option<ProblemResult> {
        let i = self.current;
        self.current = self.current + 1;
        self.solve_problem(i)
    }
}

