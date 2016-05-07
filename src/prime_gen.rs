fn qmod(mut b:u64,mut e:u64,m:u64)->u64{
	let mut result = 1;
	b = b % m;
	while e > 0 {
		if e % 2 == 1 {
			result = (result * b) % m;
		}
		e = e >> 1;
		b = (b * b) % m;
	}
	result
}

//https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test
pub fn is_prime(n:u64)->bool{
	if n == 1 || n == 4 { return false }
	if n == 2 || n == 3 || n == 5 { return true }
	if n % 2 == 0 { return false }
	
	let mut s = 0;
	let mut d = n-1;
	while d % 2 == 0 { s += 1; d /= 2; }
	
	let mut nums = Vec::new();
	if n < 2047 {
		nums.push(2);
	}else if n < 1_373_653 {
		nums.push(2);
		nums.push(3);
	}else if n < 9_080_191 {
		nums.push(31);
		nums.push(73);
	}else if n < 25_326_001 {
		nums.push(2);
		nums.push(3);
		nums.push(5);
	}else if n < 4_759_123_141 {
		nums.push(2);
		nums.push(7);
		nums.push(61);
	}else{ //if n < 1_122_004_669_633
		nums.push(2);
		nums.push(13);
		nums.push(23);
		nums.push(1_662_803);
	}
	
	for a in nums {
		let mut x = qmod(a,d,n);
		if x == 1 || x == n - 1 { continue; }
		for _ in 0..s-1 {
			x = (x * x) % n;
			if x == 1 { return false }
			if x == n - 1 { break; }		
		}
		if x == n - 1 { continue; }
		return false
	}	
	true
}

pub fn contains(num:u64,v:&Vec<u64>) -> bool {
	if v.len() == 0 { return false }
	let mut l = 0;
	let mut h = v.len() - 1;
	while l <= h {
		let m = (h + l) / 2;
		if v[m] == num {
			return true
		}else if v[m] < num {
			l = m + 1;
		}else if num < v[m] {
			if m == 0 {
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
