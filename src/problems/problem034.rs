
pub fn can(n:u64,facts:[usize;10]) -> bool {
	let mut sum = 0;
	let mut t = n;
	while t > 0 {
		sum += facts[(t%10) as usize];
		t /= 10;
	}
	sum as u64 == n
}

pub fn go() -> String {
	let mut facts = [0;10];
	facts[0] = 1;
	for i in 1..10 {
		facts[i] = i * facts[i-1];
	}
	let ubound = 9 * facts[9];
	let mut sum = 0;
	for i in 10..ubound {
		if can(i as u64,facts) {
			sum += i;
		}
	}
	sum.to_string()
}

problem!(go);
