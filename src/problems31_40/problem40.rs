fn to_digs(mut n:u64)->Vec<u8> {
	let mut d : Vec<u8> = Vec::new();
	while n > 0 {
		d.push((n%10) as u8);
		n/=10;
	}
	d.reverse();
	d
}

pub fn go() -> u64 {
	let mut frac : Vec<u8> = Vec::new();
	frac.push(0);
	let mut i = 1;
	while frac.len() <= 1_000_000 {
		for &j in to_digs(i).iter() {
			frac.push(j);
		}
		i+=1;
	}
	(frac[1] * frac[10] * frac[100] * frac[1000] * frac[10_000] * frac[100_000] * frac[1_000_000]) as u64
}