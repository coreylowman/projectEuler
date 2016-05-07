#[macro_use(problem)]
extern crate util;

fn has(n:u64)->bool{
	let mut digs = [0;10];
	let mut q = n;
	while q > 0 {
		digs[(q%10) as usize] += 1;
		q /= 10;
	}
	for x in 2..7 {
		q = x * n;
		let mut qdigs = [0;10];
		while q > 0 {
			qdigs[(q%10) as usize] += 1;
			q /= 10;
		}
		if qdigs != digs {
			return false
		}
	}
	true
}


pub fn go() -> String {
	let mut i = 125874u64;
	loop {
		if has(i) {
			return i.to_string()
		}
		i+=1;
	}
}

problem!(go);
