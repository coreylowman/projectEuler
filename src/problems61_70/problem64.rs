fn get_period(n : f32)-> usize {
	if n.sqrt() == n.sqrt().floor() {
		return 0
	}
	let mut sub : f32 = n.sqrt().floor();
	let mut mul : f32 = 1f32;
	let mut seq = Vec::new();
	let mut new_seq = Vec::new();
	let mut next_match = 0;
	loop{
		let x = (mul * (n.sqrt() + sub) / (n - sub * sub)).floor();
		mul = (n - (sub * sub)) / mul;		
		sub = x * mul - sub;

		if next_match < seq.len() && seq[next_match] == (x,sub,mul) {
			new_seq.push((x,sub,mul));
			next_match += 1;
		}else {
			next_match = 0;
			for &q in new_seq.iter(){
				seq.push(q);
			}
			seq.push((x,sub,mul));
			new_seq.clear();
		}
		
		if seq == new_seq {
			return seq.len()
		}
	}
}

pub fn go() -> u64 {
	let mut num = 0;
	for i in 2..10_001 {
		if get_period(i as f32) % 2 == 1 {
			num += 1;
		}
	}
	num
}