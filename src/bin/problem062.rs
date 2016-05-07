#[macro_use(problem)]
extern crate util;

fn get_digs(mut n : u64) -> [u8;10] {
	let mut digs = [0u8;10];
	while n > 0 {
		digs[(n%10) as usize] += 1;
		n /= 10;
	}
	digs
}

pub fn go() -> String {
	let vals : Vec<u64> = (1..10_000).map(|x| x * x * x).collect();
	for i in 0..vals.len(){
		let n3 = vals[i];
		let l = n3.to_string().len();
		let mut num = 0;
		let digs = get_digs(n3);
		let mut j = i + 1;
		loop {
			if j >= vals.len() { break }
			let q3 = vals[j];
			if n3 == q3 { continue }
			if q3.to_string().len() > l { break }
			if digs == get_digs(q3) {
				num += 1;
			}
			j += 1;
		}
		if num == 4 { println!("{}",i); return n3.to_string() }
	}
	0.to_string()
}

problem!(go);
