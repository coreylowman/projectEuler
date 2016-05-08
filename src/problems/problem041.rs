use util::prime_gen;

fn num_length(mut a:u64) -> usize {
	let mut sum = 0;
	while a > 0 {
		sum+=1;
		a /= 10;
	}
	sum
}

fn has_all(mut a:u64) -> bool{
	let s = num_length(a) + 1;
	let mut digs = vec![0;s];
	
	while a > 0 {
		let q = (a % 10) as usize;
		if q >= s || digs[q] == 1  || q == 0 { return false }
		digs[q] += 1;
		a /= 10;
	}
	
	true
}

fn go() -> String {
	//a number with 9 or 8 digits that is pandigital is divisible by 3
	//as 1 +...+9 = 45 % 3 == 0
	//and 1 + ... + 8 = 36 % 3 == 0
	let max : u64 = 7_654_321;
	let primes = prime_gen::gen(max as i64);
	let mut i : u64 = max;
	loop {
		if has_all(i) && i % 2 != 0 && i % 3 != 0 && i % 5 != 0 {
			if primes.binary_search(&i).is_ok() {
				return i.to_string()
			}
		}
		if i == 0 {  return 0.to_string() }
		i -= 1
	}
}

problem!(go, 7652413);
