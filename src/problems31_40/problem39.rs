fn gcd(a:u64,b:u64) -> u64{
	if b == 0 {
		return a
	}
	gcd(b,a%b)
}

pub fn go() -> u64 {
	let mut ps = [0;1001];
	for n in 1..100 {
		for m in (n+1)..100 {
			if (m - n) % 2 == 1 && gcd(m,n) == 1 {
				for k in 1..100 {
					let a = k * (m * m - n * n);
					let b = k * 2 * m * n;
					let c = k * (m * m + n * n);
					if a + b + c <= 1000 {
						ps[(a+b+c) as usize] += 1;
					}
				}
			}
		}
	}
	let mut m = 0;
	for i in 0..1001 {
		if ps[i] > ps[m] {
			m = i;
		}
	}
	m as u64
}