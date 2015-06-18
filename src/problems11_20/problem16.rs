pub fn go() -> u64{	
	let mut n : Vec<u64> = vec![1];	
	for _ in 0..1000 {
		let mut carries : Vec<u64> = vec![0;n.len() + 1];
		for j in 0..n.len() {
			let x = 2*n[j];
			if x >= 10 {
				n[j] = x - 10;
				carries[j+1] += 1;
			}else{
				n[j] = x;
			}
		}
		for j in 0..carries.len() {
			if j >= n.len() && carries[j] > 0 {
				n.push(carries[j]);	
			}else if j < n.len(){
				n[j] += carries[j];
			}
		}
	}
	let mut sum : u64 = 0;
	for x in n.iter() {
		sum = sum + x;
	}
	sum
}