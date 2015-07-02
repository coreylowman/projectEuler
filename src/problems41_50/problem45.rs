use prime_gen;
 
fn t(n:u64) -> u64 {
	n * (n+1) / 2
}
 
fn p(n:u64) -> u64 {
	n *(3*n - 1) / 2
}
 
fn h(n:u64) -> u64 {
	n * (2 * n - 1)
}
 
pub fn go() -> u64 {
	let mut ts = Vec::new();
	let mut ps = Vec::new();
	let mut hs = Vec::new();
   
	for i in 0..100000 {
		ts.push(t(i));
		ps.push(p(i));
		hs.push(h(i));
	}
   
	for i in 286..100000 {
		if prime_gen::contains(ts[i],&ps) && prime_gen::contains(ts[i],&hs) {
			return ts[i]
		}
	}
	0
}