extern crate num;

fn half(n:u64) -> u64 {
	n / 2
}

fn t(n:u64) -> u64 {
	half(n) * (n + 1u64)
}
 
fn p(n:u64) -> u64 {
	let half = half(n);
	3u64 * n * half - half
}
 
fn h(n : u64) -> u64 {
	n * (2u64 * n - 1u64)
}
 
fn go() -> String {
	let mut ts = Vec::new();
	let mut ps = Vec::new();
	let mut hs = Vec::new();

	for i in 0..100000 {
		ts.push(t(i));
		ps.push(p(i));
		hs.push(h(i));
	}
   
	for i in 286..100000 {
		if ps.binary_search(&ts[i]).is_ok() && hs.binary_search(&ts[i]).is_ok() {
			return ts[i].to_string()
		}
	}
	0.to_string()
}

problem!(go, 1533776805);
