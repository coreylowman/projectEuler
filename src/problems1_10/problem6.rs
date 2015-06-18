pub fn go() -> u64 {
	let mut sum = 0;
	let mut sumsq = 0;
	for i in 1..101 {
		sum += i;
		sumsq += i * i;
	}
	sum * sum - sumsq
}