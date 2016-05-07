#[macro_use(problem)]
extern crate util;

fn get_digits(mut n:u64) -> Vec<u64> {
	let mut res :Vec<u64> = Vec::new();
	while n > 0 {
		res.push(n%10);
		n /= 10;
	}
	res
}

fn can_be(n :u64) -> bool {
	let digs = get_digits(n);
	let mut sum = 0;
	for x in digs.iter() {
		sum += x.pow(5);
	}
	sum == n
}

pub fn go() -> String {
	let mut sum = 0;
	for i in 2..1_000_000 {
		if can_be(i) {
			sum += i;
		}
	}
	sum.to_string()
}

problem!(go);
