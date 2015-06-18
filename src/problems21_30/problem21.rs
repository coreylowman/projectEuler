fn d(n:u64) -> u64 {	
	let mut sum = 0;
	for i in 1..n {
		if n % i == 0 {
			sum += i;
		}	
	}
	sum
}

pub fn go() -> u64 {
	let mut sum = 0;
	for a in 2..10000 {
		let b = d(a);
		let s = d(b);
		if a != b && s == a {
			sum += a;
		}
	}
	sum
}