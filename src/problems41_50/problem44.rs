use prime_gen;

fn p(n:u64) -> u64 { (3 * n * n - n) / 2 }
 
pub fn go() -> u64{
	let mut ps = Vec::new();
	ps.push(0);
	
	for i in 1..5_000 { ps.push(p(i)); }
   
	for i in 1..ps.len() {
		let d = ps[i];
		for j in 1..ps.len() {
			let pj = ps[j];
			let pk = d + pj;
			if prime_gen::contains(pk,&ps) && prime_gen::contains(pk + pj,&ps) {
				return d
			}
		}   
	}
	0
}