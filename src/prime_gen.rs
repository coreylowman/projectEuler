pub fn contains(num:u64,v:&Vec<u64>) -> bool {
	let mut l = 0;
	let mut h = v.len() - 1;
	while l <= h {
		let m = (h + l) / 2;
		if v[m] == num {
			return true
		}else if v[m] < num {
			l = m + 1;
		}else if num < v[m] {
			if h == 0 {
				return false
			}
			h = m - 1;
		}
	}
	return false
}

pub fn icontains(num:i64,v:&Vec<i64>) -> bool {
	let mut l = 0;
	let mut h = v.len() - 1;
	while l <= h {
		let m = (h + l) / 2;
		if v[m] == num {
			return true
		}else if v[m] < num {
			l = m + 1;
		}else if num < v[m] {
			if h == 0 {
				return false
			}
			h = m - 1;
		}
	}
	return false
}

pub fn igen(limit:i64) -> Vec<i64> {
	let flim = limit as f64;
	let lim_sqrt = (flim.sqrt() + 1.0) as i64;
	
	let mut sieve = vec![false;limit as usize + 1];
	sieve[0] = false;
	sieve[1] = false;
	sieve[2] = true;
	sieve[3] = true;
	
	for x in 1..lim_sqrt {
		for y in 1..lim_sqrt {			
			let mut n = 4 * x * x + y * y;
			if n <= limit && (n % 12 == 1 || n % 12 == 5) {
				sieve[n as usize] = !sieve[n as usize];
			}
			
			n = 3 * x * x + y * y;
			if n <= limit && n % 12 == 7 {
				sieve[n as usize] = !sieve[n as usize];
			}
			
			n = 3 * x * x - y * y;
			if x > y && n <= limit && n % 12 == 11 {
				sieve[n as usize] = !sieve[n as usize];
			}
		}
	}
	
	for n in 5..lim_sqrt {
		if sieve[n as usize] {
			let x = n * n;
			let mut i = x;
			while i < limit + 1 {
				sieve[i as usize] = false;
				i += x;
			}
		}
	}
	let mut primes : Vec<i64> = Vec::new();
	for i in 0..(limit+1) {
		if sieve[i as usize] {
			primes.push(i as i64);
		}
	}
	primes
}

pub fn gen(limit : i64) -> Vec<u64> {
	let flim = limit as f64;
	let lim_sqrt = (flim.sqrt() + 1.0) as i64;
	
	let mut sieve = vec![false;limit as usize + 1];
	sieve[0] = false;
	sieve[1] = false;
	sieve[2] = true;
	sieve[3] = true;
	
	for x in 1..lim_sqrt {
		for y in 1..lim_sqrt {			
			let mut n = 4 * x * x + y * y;
			if n <= limit && (n % 12 == 1 || n % 12 == 5) {
				sieve[n as usize] = !sieve[n as usize];
			}
			
			n = 3 * x * x + y * y;
			if n <= limit && n % 12 == 7 {
				sieve[n as usize] = !sieve[n as usize];
			}
			
			n = 3 * x * x - y * y;
			if x > y && n <= limit && n % 12 == 11 {
				sieve[n as usize] = !sieve[n as usize];
			}
		}
	}
	
	for n in 5..lim_sqrt {
		if sieve[n as usize] {
			let x = n * n;
			let mut i = x;
			while i < limit + 1 {
				sieve[i as usize] = false;
				i += x;
			}
		}
	}
	let mut primes :Vec<u64> = Vec::new();
	for i in 0..(limit+1) {
		if sieve[i as usize] {
			primes.push(i as u64);
		}
	}
	primes
}