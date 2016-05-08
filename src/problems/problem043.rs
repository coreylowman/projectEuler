use util::numbers::next_perm;

fn property(digs : &Vec<u8>) -> bool {
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

fn to_num(digs : &Vec<u8>)-> u64 {
	let mut num : u64 = 0;
	for &d in digs.iter() {
		num *= 10;
		num += d as u64;
	}
	num
}

fn go() -> String{
	let mut sum :u64 = 0;
	let mut start = vec![0,1,2,3,4,5,6,7,8,9];
	loop{
		if property(&start) {
			sum += to_num(&start);
		}
		if !next_perm(&mut start) {
			break;
		}
	}
	sum.to_string()
}

problem!(go, 16695334890);
