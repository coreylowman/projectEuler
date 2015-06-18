fn gcd(a:u64,b:u64) -> u64{
	if b == 0 {
		return a
	}
	gcd(b,a%b)
}

//generate all pythagorean triples according to
//http://en.wikipedia.org/wiki/Pythagorean_triple
pub fn go() -> u64{
	for n in 1..100 {
		for m in (n+1)..100 {
			if (m - n) % 2 == 1 && gcd(m,n) == 1 {
				for k in 1..100 {
					let a = k * (m * m - n * n);
					let b = k * 2 * m * n;
					let c = k * (m * m + n * n);
					if a + b + c == 1000{
						return a*b*c;
					}
				}
			}
		}
	}
	0
}