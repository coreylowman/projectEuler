//https://en.wikipedia.org/wiki/Permutation#Generation_in_lexicographic_order
fn next_perm(mut v : [u8;10])->Option<[u8;10]>{
	let vl = v.len();	
	let mut i = vl-2;
	let mk;
	loop{
		if v[i] < v[i+1] {
			mk = i;
			break;
		}
		if i == 0 {
			return None
		}
		i-=1;
	}
	let mut ml = 0;
	i = vl-1;
	while i > mk {
		if v[mk] < v[i] {
			ml = i;
			break;
		}
		i-=1;
	}
	let mut t = v[mk];
	v[mk] = v[ml];
	v[ml] = t;
	
	i = mk + 1;
	let mut j = vl-1;
	while i < j {
		t = v[i];
		v[i] = v[j];
		v[j] = t;
		i+=1;
		j-=1;
	}
	Some(v)
}

fn property(digs : [u8;10])->bool{	
	if digs[5] != 5 {
		return false
	}
	let arr = [2,3,5,7,11,13,17];	
	for i in 1..8 {
		let d1 = digs[i] as u32;
		let d2 = digs[i+1] as u32;
		let d3 = digs[i+2] as u32;
		let num = 100 * d1 + 10 * d2 + d3;
		if num % arr[i-1] != 0 {
			return false
		}
	}
	true
}

fn to_num(digs : [u8;10])-> u64 {
	let mut num : u64 = 0;
	for &d in digs.iter() {
		num *= 10;
		num += d as u64;
	}
	num
}

fn go() -> String{
	let mut sum :u64 = 0;
	let mut start : [u8;10] = [0,1,2,3,4,5,6,7,8,9];
	loop{
		if property(start.clone()) {
			sum += to_num(start.clone());
		}
		match next_perm(start) {
			Some(v) => start = v,
			None => break,
		}
	}
	sum.to_string()
}

problem!(go, 16695334890);
