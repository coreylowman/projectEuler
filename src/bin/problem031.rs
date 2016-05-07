#[macro_use(problem)]
extern crate util;

fn f(t:u64,c:usize) -> u64 {
	let cs = [1,2,5,10,20,50,100,200];
	if t == 0 || c == 0 {
		return 1
	}else {
		let mut i = 0;
		let mut sum = 0;
		while i <= t/cs[c] {
			sum += f(t-i*cs[c],c-1);
			i += 1;
		}
		return sum
	}
}

pub fn go() -> String {
	f(200,7).to_string()
}

problem!(go);
